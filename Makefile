# Set target dir to default if not set in env
CARGO_TARGET_DIR ?= ./target

all: build start


build:
	cargo build --release --target=wasm32-unknown-unknown
	wasm-bindgen ${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/cursed_rust.wasm --target web --out-dir pkg

debug:
	RUSTFLAGS=-g cargo build --release --target=wasm32-unknown-unknown
	wasm-bindgen ${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/cursed_rust.wasm --target web --out-dir pkg

start:
	python3 -m http.server 8124

