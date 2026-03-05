.PHONY: build release test mcp-server run run-sse clean

build:
	cd engine && cargo build

release:
	cd engine && cargo build --release

test:
	cd engine && cargo test

mcp-server: release
	GRAPH_DIR=./graph/math PROGRESS_DIR=./progress \
		./engine/target/release/open-mastery-mcp

run: release
	GRAPH_DIR=./graph/math PROGRESS_DIR=./progress \
		./engine/target/release/open-mastery-mcp --stdio

run-sse: release
	GRAPH_DIR=./graph/math PROGRESS_DIR=./progress \
		./engine/target/release/open-mastery-mcp

clean:
	cd engine && cargo clean
