OUT_DIR := build
OUT_BIN := ${OUT_DIR}/main

.PHONY: all
all: build run

.PHONY: build
build: ${OUT_BIN}

.PHONY: run
run: ${OUT_BIN}
	mkdir -p gen/data
	./${OUT_BIN}

${OUT_BIN}: src/main.go
	go build -o ./${OUT_BIN} -gcflags "-l" ./src/main.go
