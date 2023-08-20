# Install wasm target and http-server
install-deps:
    rustup target add wasm32-unknown-unknown &&\
    cargo install basic-http-server

# Build for release
build:
    cargo build --release --target wasm32-unknown-unknown --target-dir build/target/ &&\
    cp assets/* build/

# Build for debug, and open http server
serve:
    cargo build --target wasm32-unknown-unknown --target-dir build/target/ &&\
    cp assets/* build/ &&\
    basic-http-server build

