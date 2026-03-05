.PHONY: build release test run-student run-student-sse run-teacher run-teacher-sse clean

build:
	cd engine && cargo build

release:
	cd engine && cargo build --release

test:
	cd engine && cargo test

run-student: release
	GRAPH_DIR=./graph/math PROGRESS_DIR=./progress \
		./engine/target/release/open-mastery-student --stdio

run-student-sse: release
	GRAPH_DIR=./graph/math PROGRESS_DIR=./progress \
		./engine/target/release/open-mastery-student

run-teacher: release
	GRAPH_DIR=./graph/math REPO_DIR=. \
		./engine/target/release/open-mastery-teacher --stdio

run-teacher-sse: release
	GRAPH_DIR=./graph/math REPO_DIR=. \
		./engine/target/release/open-mastery-teacher

clean:
	cd engine && cargo clean
