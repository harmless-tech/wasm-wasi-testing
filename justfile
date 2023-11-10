default:
    just -l

build:
    cargo build -p wasmmm --target wasm32-wasi
    wasm-tools component new ./target/wasm32-wasi/debug/wasmmm.wasm \
        -o ./target/wasm32-wasi/debug/wasmmm-comp.wasm \
        --adapt ./wasi/wasi_snapshot_preview1.wasm
    cargo build -p wasi-test

run:
    just build
    cargo run -p wasi-test

buildr:
    cargo build -p wasmmm --target wasm32-wasi --release
    wasm-tools component new ./target/wasm32-wasi/debug/wasmmm.wasm \
        -o ./target/wasm32-wasi/debug/wasmmm-comp.wasm \
        --adapt ./wasi/wasi_snapshot_preview1.wasm
    cargo build -p wasi-test --release

runr:
    just buildr
    cargo run -p wasi-test --release
