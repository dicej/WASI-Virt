cargo +nightly build -p virtual-adapter --target wasm32-wasi --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort && cp target/wasm32-wasi/release/virtual_adapter.wasm lib/
cargo +nightly build -p virtual-adapter --target wasm32-wasi --release --features debug -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort && cp target/wasm32-wasi/release/virtual_adapter.wasm lib/virtual_adapter.debug.wasm

cargo +nightly build -p virt-to-wasi-adapter --target wasm32-wasi --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort && cp target/wasm32-wasi/release/virt_to_wasi_adapter.wasm lib/
cargo +nightly build -p wasi-to-virt-adapter --target wasm32-wasi --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort && cp target/wasm32-wasi/release/wasi_to_virt_adapter.wasm lib/
