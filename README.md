practice 1
================================
`https://guptanikhil.medium.com/rust-wasm-breakout-game-i-1b474e41a132`




bevy
================================
`cargo add wasm-bindgen`

`cargo add bevy`


WASM Build
================================
- Cargo.toml 디렉토리 경로 : wasm 디렉토리 있어야됨

`cargo build --target wasm32-unknown-unknown --release`

`wasm-bindgen target/wasm32-unknown-unknown/release/{wasm 이름}.wasm --out-dir wasm --target web`