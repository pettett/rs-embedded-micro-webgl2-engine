use gltf::{
    buffer::{self, Data},
    Document, Error,
};
use std::collections::HashMap;
use web_sys::{console, WebGl2RenderingContext};

use crate::render::rgl::texture::Tex;
pub struct GltfMesh {
    pub doc: Document,
    pub buffers: Vec<Data>,
}

pub struct Assets {
    gltf: HashMap<String, GltfMesh>,
    textures: HashMap<String, std::rc::Rc<Tex>>,
    error_tex: Option<std::rc::Rc<Tex>>,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            gltf: HashMap::new(),
            textures: HashMap::new(),
            error_tex: None,
        }
    }

    pub fn load(&mut self, gl: &WebGl2RenderingContext) {
        self.error_tex = Some(std::rc::Rc::new(Tex::new_error(gl)));
    }

    /// Import the buffer data referenced by a glTF document.
    pub fn import_buffer_data(
        document: &Document,
        mut blob: Option<Vec<u8>>,
    ) -> Result<Vec<buffer::Data>, Error> {
        let mut buffers = Vec::new();
        for buffer in document.buffers() {
            let mut data = match buffer.source() {
                buffer::Source::Uri(uri) => todo!(),
                buffer::Source::Bin => blob.take().ok_or(Error::MissingBlob),
            }?;
            if data.len() < buffer.length() {
                return Err(Error::BufferLength {
                    buffer: buffer.index(),
                    expected: buffer.length(),
                    actual: data.len(),
                });
            }
            while data.len() % 4 != 0 {
                data.push(0);
            }
            buffers.push(buffer::Data(data));
        }
        Ok(buffers)
    }

    pub fn load_gltf(&mut self, gltf_name: String, gltf: &[u8]) -> Result<(), gltf::Error> {
        let doc = gltf::Gltf::from_slice(gltf)?;
        let buffers = Self::import_buffer_data(&doc.document, doc.blob)?;

        self.gltf.insert(
            gltf_name,
            GltfMesh {
                doc: doc.document,
                buffers,
            },
        );
        Ok(())
    }

    pub fn register_tex(&mut self, tex_name: String, tex: Tex) {
        self.textures.insert(tex_name, std::rc::Rc::new(tex));
    }

    pub fn get_tex(&self, tex_name: &str) -> std::rc::Rc<Tex> {
        if let Some(t) = self.textures.get(tex_name) {
            return t.clone();
        } else {
            // Return the error texture
            console::warn_1(&format!("{} is not a loaded texture", tex_name).into());
            return self.error_tex.clone().expect("Error texture not loaded!");
        }
    }

    pub fn get_gltf(&self, gltf_name: &str) -> Option<&GltfMesh> {
        self.gltf.get(gltf_name)
    }
}
