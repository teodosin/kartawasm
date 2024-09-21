rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install wasm-bindgen-cli

cd rustwasm
cargo build --release --bin rustwasm --target wasm32-unknown-unknown
# cargo build --release --bin rustwasm --target wasm32-wasi

cd ../


wasm-bindgen --out-dir sveltefront/src/assets/wasm_client --target web ./rustwasm/target/wasm32-unknown-unknown/release/rustwasm.wasm
