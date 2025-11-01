 Makefile for Rust Coins Docker Project

# Variables
IMAGE_NAME = coins
CONTAINER_NAME = coins-app
VERSION = latest
DOCKER_TAG = $(IMAGE_NAME):$(VERSION)

# Default target
.DEFAULT_GOAL := help

# Help target
.PHONY: help
help:
	@echo "Rust Coins - Docker Management"
	@echo "=============================="
	@echo ""
	@echo "Available targets:"
	@echo "  make build        - Build Docker image"
	@echo "  make run          - Run the coins application in Docker"
	@echo "  make test         - Run tests inside Docker"
	@echo "  make shell        - Open interactive shell in container"
	@echo "  make clean        - Remove Docker images and containers"
	@echo "  make rebuild      - Clean and rebuild from scratch"
	@echo "  make release      - Build optimized production image"
	@echo "  make logs         - View container logs"
	@echo "  make inspect      - Inspect the Docker image"
	@echo ""

## Builds the Docker image
.PHONY: build
build:
	@echo "Building Docker image: $(DOCKER_TAG)"
	docker build -t $(DOCKER_TAG) .

# Run the application
.PHONY: run
run:
	@echo "Running coins application..."
	docker run --rm --name $(CONTAINER_NAME) $(DOCKER_TAG)

# Run tests inside Docker
.PHONY: test
test:
	@echo "Running tests in Docker..."
	docker run --rm $(DOCKER_TAG) cargo test

# Open interactive shell
.PHONY: shell
shell:
	@echo "Opening shell in container..."
	docker run --rm -it --entrypoint /bin/bash $(DOCKER_TAG)

# Build shell - access builder environment with Rust toolchain
.PHONY: build-shell
build-shell:
	@echo "Opening shell in builder environment..."
	docker run --rm -it \
		-v $(PWD):/usr/src/coins \
		-w /usr/src/coins \
		rust:1.83 \
		/bin/bash

# Clean up Docker resources
.PHONY: clean
clean:
	@echo "Cleaning up Docker resources..."
	-docker rm -f $(CONTAINER_NAME) 2>/dev/null || true
	-docker rmi $(DOCKER_TAG) 2>/dev/null || true
	@echo "Cleanup complete"

# Rebuild from scratch
.PHONY: rebuild
rebuild: clean build
	@echo "Rebuild complete"

# Build release version
.PHONY: release
release:
	@echo "Building optimized production image..."
	docker build --no-cache -t $(IMAGE_NAME):release .

# View logs
.PHONY: logs
logs:
	@echo "Viewing container logs..."
	docker logs $(CONTAINER_NAME)

# Inspect the image
.PHONY: inspect
inspect:
	@echo "Inspecting Docker image: $(DOCKER_TAG)"
	@docker images $(IMAGE_NAME)
	@echo ""
	@echo "Image details:"
	@docker inspect $(DOCKER_TAG) | grep -E '"Size"|"Created"'

# Run in detached mode
.PHONY: run-detached
run-detached:
	@echo "Running coins application in background..."
	docker run -d --name $(CONTAINER_NAME) $(DOCKER_TAG)

# Local cargo commands (requires Rust installed locally)
.PHONY: local-build
local-build:
	@echo "Building locally with cargo..."
	cargo build --release

.PHONY: local-run
local-run:
	@echo "Running locally with cargo..."
	cargo run --release

.PHONY: local-test
local-test:
	@echo "Running tests locally with cargo..."
	cargo test
