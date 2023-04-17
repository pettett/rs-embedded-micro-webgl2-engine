pub mod mark_requirements_mesh;
pub mod mark_requirements_trait;
pub mod mark_requirements_water;

use gltf::{
    buffer::{self, Data},
    Document, Error,
};
use rhai::Locked;
use std::collections::{HashMap, HashSet};
use web_sys::WebGl2RenderingContext;

use crate::{fetch, load_texture_img::load_texture_image, render::rgl::texture::Tex};

use self::mark_requirements_trait::MarkRequirements;

use super::store::Store;

pub struct GltfMesh {
    pub doc: Document,
    pub buffers: Vec<Data>,
}

pub struct Assets {
    gltf: HashMap<String, GltfMesh>,
    loading_gltf: HashSet<String>,
    textures: HashMap<String, std::rc::Rc<Tex>>,
    loading_textures: HashSet<String>,
    error_tex: Option<std::rc::Rc<Tex>>,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            gltf: HashMap::new(),
            loading_gltf: HashSet::new(),
            textures: HashMap::new(),
            loading_textures: HashSet::new(),
            error_tex: None,
        }
    }
    pub fn mark_requirements(&mut self, store: &Store) {
        for e in &store.state.entities {
            match &**e {
                super::store::Entity::EntMesh(m) => m.borrow().mark_requirements(self),
                super::store::Entity::EntWater(w) => (w.mark_requirements(self)),
            }
        }
    }
    pub async fn load_requirements(
        assets: std::rc::Rc<std::cell::RefCell<Self>>,
        gl: std::rc::Rc<WebGl2RenderingContext>,
    ) {
        let l = {
            let mut a = assets.borrow_mut();
            let t = a.loading_gltf.clone();
            a.loading_gltf = HashSet::new();
            t
        };

        for gltf in l {
            if !assets.borrow().gltf.contains_key(&gltf) {
                let data = fetch::fetch(&gltf).await.unwrap();

                assets.borrow_mut().load_gltf(gltf, &data[..]).unwrap();
            }
        }

        let l = {
            let mut a = assets.borrow_mut();
            let t = a.loading_textures.clone();
            a.loading_textures = HashSet::new();
            t
        };

        for tex in l {
            if !assets.borrow().textures.contains_key(&tex) {
                load_texture_image(gl.clone(), assets.clone(), tex);
            }
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

    pub fn require_gltf(&mut self, gltf: String) {
        self.loading_gltf.insert(gltf);
    }
    pub fn require_texture(&mut self, tex: String) {
        self.loading_textures.insert(tex);
    }

    pub fn register_tex(&mut self, tex_name: String, tex: Tex) {
        self.textures.insert(tex_name, std::rc::Rc::new(tex));
    }

    pub fn get_tex(&self, tex_name: &str) -> std::rc::Rc<Tex> {
        if let Some(t) = self.textures.get(tex_name) {
            return t.clone();
        } else {
            // Return the error texture
            log::warn!("{} is not a loaded texture", tex_name);
            return self.error_tex.clone().expect("Error texture not loaded!");
        }
    }

    pub fn get_gltf(&self, gltf_name: &str) -> Option<&GltfMesh> {
        self.gltf.get(gltf_name)
    }
}
