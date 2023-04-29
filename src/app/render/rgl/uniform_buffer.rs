use std::marker::PhantomData;
use std::mem;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use super::shader::Shader;

pub struct UniformBuffer<T> {
    buffer: Option<WebGlBuffer>,
    ph: PhantomData<T>,
}

impl<T: Sized> UniformBuffer<T> {
    pub fn new(gl: &GL) -> UniformBuffer<T> {
        let ubo_block = gl.create_buffer();
        gl.bind_buffer(GL::UNIFORM_BUFFER, ubo_block.as_ref());

        let d = vec![0u8; mem::size_of::<T>()];

        gl.buffer_data_with_u8_array(GL::UNIFORM_BUFFER, &d[..], GL::DYNAMIC_DRAW); // allocate memory

        gl.bind_buffer(GL::UNIFORM_BUFFER, None);

        UniformBuffer {
            buffer: ubo_block,
            ph: PhantomData,
        }
    }

    ///https://stackoverflow.com/a/42186553
    unsafe fn data_as_u8_slice(p: &T) -> &[u8] {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }

    pub fn buffer(&self, gl: &GL, data: &T) {
        let d = unsafe { Self::data_as_u8_slice(data) };

        let data_array = super::to_array_buffer_view(d);

        gl.bind_buffer(GL::UNIFORM_BUFFER, self.buffer.as_ref());
        //buffer memory - use sub data to stop reallocation

        gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 0, &data_array);
    }
    /// Bind uniform buffer for use in shaders
    pub fn bind_base(&self, gl: &GL, shader: &Shader, block_index: u32, binding_point: u32) {
        gl.bind_buffer_base(GL::UNIFORM_BUFFER, binding_point, self.buffer.as_ref());
        gl.uniform_block_binding(&shader.program, block_index, binding_point);
    }
}
