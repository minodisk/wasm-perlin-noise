rustc \
  +nightly \
  --target wasm32-unknown-unknown \
  --crate-type cdylib \
  -O src/lib.rs \
  --out-dir dist
