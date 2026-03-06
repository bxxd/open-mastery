.PHONY: build release install test validate run-student run-student-sse run-teacher run-teacher-sse clean

build:
	cd engine && cargo build

release:
	cd engine && cargo build --release

install: release
	@echo "Binaries built:"
	@echo "  ./engine/target/release/open-mastery-student"
	@echo "  ./engine/target/release/open-mastery-teacher"

test:
	cd engine && cargo test

validate:
	@cd engine && cargo test --quiet 2>&1 && echo "All tests passed." || (echo "Tests failed!" && exit 1)
	@echo "Graph: $$(find graph/math -name '*.yaml' ! -name '_prompt.yaml' | wc -l) nodes, $$(find graph/math -name '_prompt.yaml' | wc -l) prompts"

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
