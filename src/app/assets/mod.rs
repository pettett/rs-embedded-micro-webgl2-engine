use gltf::{
    buffer::{self, Data},
    image::Source,
    Document, Error,
};
use std::collections::{HashMap, HashSet};
use web_sys::WebGl2RenderingContext;

use crate::{
    fetch,
    render::{material::MatAlbedo, rgl::texture::Tex},
};

use super::store::Store;

pub struct GltfMesh {
    pub doc: Document,
    pub buffers: Vec<Data>,
}
struct AssetStore<T> {
    assets: Vec<Option<T>>,
    asset_indexes: HashMap<String, usize>,
    loading_assets: HashSet<String>,
}

impl<T> AssetStore<T> {
    pub fn require(&mut self, asset_name: String) -> usize {
        if let Some(a) = self.asset_indexes.get(&asset_name) {
            //This asset has already been assigned an index, and may even already be loaded
            *a
        } else {
            self.loading_assets.insert(asset_name.clone());
            let id = self.assets.len();
            self.asset_indexes.insert(asset_name, id);
            self.assets.push(None);
            id
        }
    }
    pub fn load(&mut self, asset_name: String, asset: T) {
        self.assets[self.asset_indexes[&asset_name]] = Some(asset);
    }
    /// Clear the loading asset store and return it
    pub fn consume_loading(&mut self) -> HashSet<String> {
        let l = self.loading_assets.clone();
        self.loading_assets = HashSet::new();
        l
    }

    pub fn get(&self, asset_id: usize) -> Option<&T> {
        self.assets[asset_id].as_ref()
    }
}
impl<T> Default for AssetStore<T> {
    fn default() -> Self {
        Self {
            assets: Vec::new(),
            asset_indexes: HashMap::new(),
            loading_assets: HashSet::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Mat {
    pub tex: usize,
}

pub struct Assets {
    textures: AssetStore<std::rc::Rc<Tex>>,
    gltf: AssetStore<GltfMesh>,
    materials: AssetStore<Mat>,

    error_tex: Option<std::rc::Rc<Tex>>,
}
impl Assets {
    pub fn new() -> Assets {
        Assets {
            textures: Default::default(),
            gltf: Default::default(),
            materials: Default::default(),
            error_tex: None,
        }
    }

    pub async fn load_requirements(
        assets: std::rc::Rc<std::cell::RefCell<Self>>,
        gl: std::rc::Rc<WebGl2RenderingContext>,
    ) {
        let l = assets.borrow_mut().gltf.consume_loading();

        for gltf in l {
            let data = fetch::fetch(&gltf).await.unwrap();

            let doc = gltf::Gltf::from_slice(&data[..]).unwrap();
            //TODO: Allow paths dependant on where to model is located
            let buffers = Self::import_buffer_data("assets/models", &doc.document, doc.blob)
                .await
                .unwrap();

            assets.borrow_mut().load_gltf(
                gltf,
                GltfMesh {
                    doc: doc.document,
                    buffers,
                },
            );
        }

        let l = assets.borrow_mut().textures.consume_loading();

        for tex in l {
            fetch::fetch_texture_image(gl.clone(), assets.clone(), tex);
        }
    }

    pub fn load(&mut self, gl: &WebGl2RenderingContext) {
        self.error_tex = Some(std::rc::Rc::new(Tex::new_error(gl)));
    }

    /// Import the buffer data referenced by a glTF document.
    pub async fn import_buffer_data(
        bin_path: &str,
        document: &Document,
        mut blob: Option<Vec<u8>>,
    ) -> Result<Vec<buffer::Data>, Error> {
        let mut buffers = Vec::new();
        for buffer in document.buffers() {
            let mut data = match buffer.source() {
                buffer::Source::Uri(uri) => Ok(fetch::fetch(&format!("{}/{}", bin_path, uri))
                    .await
                    .unwrap()),
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

    pub fn load_gltf(&mut self, gltf_name: String, gltf: GltfMesh) {
        self.gltf.load(gltf_name, gltf);
    }

    pub fn require_gltf(&mut self, gltf: String) -> usize {
        self.gltf.require(gltf)
    }
    pub fn require_texture(&mut self, tex: String) -> usize {
        self.textures.require(tex)
    }

    pub fn load_material(&mut self, index: usize, mat: Mat) {
        while self.materials.assets.len() <= index + 1 {
            self.materials.assets.push(None);
        }

        self.materials.assets[index] = Some(mat);
    }
    pub fn get_material(&self, mat: usize) -> &Option<Mat> {
        match self.materials.assets.get(mat) {
            Some(m) => m,
            None => &None,
        }
    }

    pub fn register_tex(&mut self, tex_name: String, tex: Tex) {
        self.textures.load(tex_name, std::rc::Rc::new(tex));
    }

    pub fn get_tex(&self, tex_name: usize) -> std::rc::Rc<Tex> {
        if let Some(t) = self.textures.get(tex_name) {
            return t.clone();
        } else {
            // Return the error texture
            //log::warn!("{} is not a loaded texture", tex_name);
            return self.error_tex.clone().expect("Error texture not loaded!");
        }
    }

    pub fn get_gltf(&self, gltf_name: usize) -> Option<&GltfMesh> {
        self.gltf.get(gltf_name)
    }
    pub fn require_mesh_textures(&mut self) {
        let mut uris = Vec::<(usize, String)>::new();

        for (path, gltf_name) in &self.gltf.asset_indexes {
            let path_head = "assets/textures/".to_owned();

            if let Some(m) = self.get_gltf(*gltf_name) {
                for mat in m.doc.materials() {
                    if let Some(Source::Uri { uri, mime_type }) = mat
                        .pbr_metallic_roughness()
                        .base_color_texture()
                        .map(|m| m.texture().source().source())
                    {
                        let mut u = path_head.clone();
                        u.push_str(uri);
                        uris.push((mat.index().unwrap(), u));
                    };
                }
            }
        }
        //TODO: This will break with more than one gltf with materials
        for (id, uri) in uris {
            let tex = self.require_texture(uri);

            self.load_material(id, Mat { tex });
        }
    }
}
