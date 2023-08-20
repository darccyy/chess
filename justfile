install-deps:
    rustup target add wasm32-unknown-unknown &&\
    cargo install basic-http-server

serve:
    cargo build --target wasm32-unknown-unknown --target-dir build/target/ &&\
    cp assets/* build/ &&\
    basic-http-server build

build:
    cargo build --target wasm32-unknown-unknown --target-dir build/target/ &&\
    cp assets/* build/

