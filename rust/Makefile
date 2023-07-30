OUT_DIR := target/release
OUT_BIN := ${OUT_DIR}/rustbigotest

.PHONY: all
all: build run

.PHONY: build
build: ${OUT_BIN}

.PHONY: run
run: ${OUT_BIN}
	mkdir -p gen/data
	./${OUT_BIN}

${OUT_BIN}: src/main.rs
	cargo rustc -r