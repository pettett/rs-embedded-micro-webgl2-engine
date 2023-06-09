pub mod texture;
pub mod vao;
use js_sys::WebAssembly;

pub mod framebuffer;
pub use framebuffer::Framebuffer;

pub mod renderbuffer;

pub mod uniform_buffer;

pub mod shader;
use wasm_bindgen::JsCast;

pub fn to_array_buffer_view(data: &[u8]) -> js_sys::Uint8Array {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let data_location = data.as_ptr() as u32;

    js_sys::Uint8Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32)
}
