
:: Build the webgl_water_tutorial.wasm file
cargo build --target wasm32-unknown-unknown --release

:: Process the webgl_water_tutorial.wasm file and generate the necessary
:: JavaScript glue code to run it in the browser.
wasm-bindgen ./target/wasm32-unknown-unknown/release/webgl_renderer.wasm --out-dir . --no-typescript --target web

wasm-opt -O3 -o optimised.wasm webgl_renderer_bg.wasm

MOVE /Y optimised.wasm webgl_renderer_bg.wasm 