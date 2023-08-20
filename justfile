# Cargo project name

# Install wasm target (for production)
install:
    rustup target add wasm32-unknown-unknown

# Install wasm target and http-server (for development)
install-dev:
    rustup target add wasm32-unknown-unknown &&\
    cargo install basic-http-server watch

# Build for release
build:
    cargo build --release --target wasm32-unknown-unknown &&\
    rm -rf build/ &&\
    mkdir build/ &&\
    cp target/wasm32-unknown-unknown/release/*.wasm build/ &&\
    cp assets/* build/

# Build for debug, and open http server
serve:
    cargo build --target wasm32-unknown-unknown &&\
    rm -rf build/ &&\
    mkdir build/ &&\
    cp target/wasm32-unknown-unknown/debug/*.wasm build/ &&\
    cp assets/* build/ &&\
    basic-http-server build

# Build for debug, and open http server, watch for changes
watch:
    cargo watch -c -s 'just serve'

