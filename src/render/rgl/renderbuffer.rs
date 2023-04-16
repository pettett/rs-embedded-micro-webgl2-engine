use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct Renderbuffer {
    pub rb: Option<WebGlRenderbuffer>,
}

impl Renderbuffer {
    pub fn new(gl: &GL, width: i32, height: i32) -> Renderbuffer {
        let rb = Renderbuffer {
            rb: gl.create_renderbuffer(),
        };

        gl.bind_renderbuffer(GL::RENDERBUFFER, rb.rb.as_ref());

        gl.renderbuffer_storage(GL::RENDERBUFFER, GL::DEPTH_COMPONENT16, width, height);

        rb
    }
    pub fn delete(&self, gl: &GL) {
        gl.delete_renderbuffer(self.rb.as_ref())
    }
}
