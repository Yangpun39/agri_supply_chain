
# To generate associated functions in did file
 cargo build --release --target wasm32-unknown-unknown --package hack1_backend

candid-extractor target/wasm32-unknown-unknown/release/hack1_backend.wasm >src/hack1_backend/hack1_backend.did

# deploy 
dfx deploy dabarcodes_backend

#option2
cargo install ic-wasm

ic-wasm target/wasm32-unknown-unknown/release/dabarcodes_backend.wasm --output src/dabarcodes_backend/dabarcodes_backend.did
