use web_sys::WebGl2RenderingContext;

pub struct Vao {
    vao: web_sys::WebGlVertexArrayObject,
}

impl Vao {
    pub fn new(gl: &WebGl2RenderingContext) -> Vao {
        let vao = gl
            .create_vertex_array()
            .ok_or("Could not create vertex array object")
            .unwrap();
        Vao { vao }
    }
    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_vertex_array(Some(&self.vao))
    }
    pub fn delete(&self, gl: &WebGl2RenderingContext) {
        gl.delete_vertex_array(Some(&self.vao))
    }
}
