
:: Build the webgl_water_tutorial.wasm file
cargo build --target wasm32-unknown-unknown

:: Process the webgl_water_tutorial.wasm file and generate the necessary
:: JavaScript glue code to run it in the browser.
wasm-bindgen ./target/wasm32-unknown-unknown/debug/webgl_renderer.wasm --out-dir . --no-typescript --target web
 