use gltf::{
    buffer::{self, Data},
    Document, Error,
};
use std::collections::HashMap;
pub struct GltfMesh {
    pub doc: Document,
    pub buffers: Vec<Data>,
}

#[derive(Default)]
pub struct Assets {
    gltf: HashMap<String, GltfMesh>,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            gltf: HashMap::new(),
        }
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

    pub fn get_gltf(&self, gltf_name: &str) -> Option<&GltfMesh> {
        self.gltf.get(gltf_name)
    }
}
