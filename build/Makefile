# Makefile for ShellMorph Pro

.PHONY: all clean core cli api

BUILD_TYPE := Release
DIST_DIR := dist

all: core cli api

core:
	@echo "Building Rust core..."
	cargo build --release --manifest-path src/core/Cargo.toml
	mkdir -p $(DIST_DIR)/core
	cp src/core/target/release/libshellmorph_core.a $(DIST_DIR)/core

cli:
	@echo "Building CLI..."
	cmake -S src/cli -B src/cli/build -DCMAKE_BUILD_TYPE=$(BUILD_TYPE)
	cmake --build src/cli/build --target shellmorph-cli
	mkdir -p $(DIST_DIR)/cli
	cp src/cli/build/shellmorph-cli $(DIST_DIR)/cli

api:
	@echo "Building API..."
	npm install --prefix src/api
	npm run build --prefix src/api
	mkdir -p $(DIST_DIR)/api
	cp -r src/api/build/* $(DIST_DIR)/api

clean:
	@echo "Cleaning build artifacts..."
	rm -rf $(DIST_DIR) src/core/target src/cli/build src/api/build
	@echo "Clean complete."
