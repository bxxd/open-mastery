use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{sse, Sse},
    routing::{get, post},
    Router,
};
use futures::stream::{Stream, StreamExt};
use serde_json::Value;
use std::{
    collections::HashMap, convert::Infallible, pin::Pin, sync::Arc, time::Duration,
};
use tokio::sync::{mpsc, RwLock};
use tower_http::cors::{Any as CorsAny, CorsLayer};
use uuid::Uuid;

use crate::server::Server;

type SseMessage = Result<sse::Event, Infallible>;
type SseSender = mpsc::UnboundedSender<SseMessage>;

struct SessionEntry {
    tx: SseSender,
}

#[derive(Clone)]
struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SessionEntry>>>,
}

impl SessionManager {
    fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn create_session(&self) -> (String, mpsc::UnboundedReceiver<SseMessage>) {
        let session_id = Uuid::new_v4().to_string();
        let (tx, rx) = mpsc::unbounded_channel();
        self.sessions
            .write()
            .await
            .insert(session_id.clone(), SessionEntry { tx });
        tracing::debug!("Created session: {}", session_id);
        (session_id, rx)
    }

    async fn send_message(&self, session_id: &str, response: Value) -> Result<(), String> {
        let sessions = self.sessions.read().await;
        let entry = sessions
            .get(session_id)
            .ok_or_else(|| format!("Session not found: {}", session_id))?;

        let json = serde_json::to_string(&response).map_err(|e| e.to_string())?;
        let event = sse::Event::default().event("message").data(json);
        entry.tx.send(Ok(event)).map_err(|e| e.to_string())
    }

    async fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
        tracing::info!("Session removed: {}", &session_id[..8.min(session_id.len())]);
    }

    async fn session_exists(&self, session_id: &str) -> bool {
        self.sessions.read().await.contains_key(session_id)
    }
}

/// Stream wrapper that removes session on drop (client disconnect).
struct CleanupStream {
    inner: Pin<Box<dyn Stream<Item = SseMessage> + Send>>,
    session_id: Option<String>,
    sessions: Option<SessionManager>,
}

impl Stream for CleanupStream {
    type Item = SseMessage;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

impl Drop for CleanupStream {
    fn drop(&mut self) {
        if let (Some(session_id), Some(sessions)) = (self.session_id.take(), self.sessions.take()) {
            tokio::spawn(async move {
                sessions.remove_session(&session_id).await;
            });
        }
    }
}

#[derive(Clone)]
struct AppState {
    sessions: SessionManager,
    server: Arc<Server>,
}

/// GET /sse — establish SSE connection
async fn handle_sse(
    State(state): State<AppState>,
) -> Result<Sse<Pin<Box<dyn Stream<Item = SseMessage> + Send>>>, StatusCode> {
    let (session_id, rx) = state.sessions.create_session().await;
    let messages_path = format!("/messages/{}", session_id);

    tracing::info!("SSE connect: session={}", &session_id[..8]);

    let endpoint_event = sse::Event::default()
        .event("endpoint")
        .data(messages_path);

    let sessions_clone = state.sessions.clone();
    let session_id_clone = session_id.clone();

    let inner = futures::stream::once(async move { Ok(endpoint_event) })
        .chain(tokio_stream::wrappers::UnboundedReceiverStream::new(rx));

    let stream: Pin<Box<dyn Stream<Item = SseMessage> + Send>> = Box::pin(CleanupStream {
        inner: Box::pin(inner),
        session_id: Some(session_id_clone),
        sessions: Some(sessions_clone),
    });

    Ok(Sse::new(stream).keep_alive(
        sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    ))
}

/// POST /messages/:session_id — handle JSON-RPC request
async fn handle_message(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    body: String,
) -> Result<StatusCode, StatusCode> {
    if !state.sessions.session_exists(&session_id).await {
        tracing::warn!("Session not found: {}", session_id);
        return Err(StatusCode::NOT_FOUND);
    }

    let request: Value = serde_json::from_str(&body).map_err(|e| {
        tracing::error!("Invalid JSON: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    let response = state.server.handle_request(request);

    if !response.is_null() {
        state
            .sessions
            .send_message(&session_id, response)
            .await
            .map_err(|e| {
                tracing::error!("Failed to send SSE: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    Ok(StatusCode::ACCEPTED)
}

pub async fn serve(server: Arc<Server>, port: u16) -> Result<()> {
    tracing::info!("Starting HTTP/SSE server on port {}", port);
    tracing::info!("  SSE endpoint: GET /sse");
    tracing::info!("  Messages: POST /messages/{{session_id}}");

    let state = AppState {
        sessions: SessionManager::new(),
        server,
    };

    let cors = CorsLayer::new()
        .allow_origin(CorsAny)
        .allow_methods(CorsAny)
        .allow_headers(CorsAny);

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/sse", get(handle_sse))
        .route("/messages/:session_id", post(handle_message))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await?;

    tracing::info!("Open Mastery MCP server ready on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
