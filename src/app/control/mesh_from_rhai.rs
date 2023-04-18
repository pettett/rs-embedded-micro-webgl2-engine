use nalgebra::{ArrayStorage, Vector3};

use crate::app::{store::Mesh, Assets};

use super::from_rhai::FromRhai;

impl FromRhai for Mesh {
    fn try_from_rhai(map: rhai::Map, assets: &mut Assets) -> Result<Mesh, &'static str> {
        let name = map["mesh"].clone().into_string().unwrap().to_owned();
        let texture = map["texture"].clone().into_string().unwrap().to_owned();

        let tex = assets.require_texture(texture);
        let mesh = assets.require_gltf(name);

        // Load the mesh if it doesnt exist already
        // if assets.borrow_mut().get_gltf(&mesh).is_none() {
        //     let data = fetch::fetch(&mesh).await.unwrap();

        //     assets
        //         .borrow_mut()
        //         .load_gltf(mesh.clone(), &data[..])
        //         .unwrap();
        // }

        let pos = match map.get("position") {
            Some(d) => super::to_vec3(d)?,
            None => [0.0; 3],
        };

        let rot = match map.get("rotation") {
            Some(d) => super::to_vec3(d)?,
            None => [0.0; 3],
        };

        let update = map
            .get("update")
            .map(|f| f.clone().cast::<rhai::FnPtr>().fn_name().to_owned());

        log::info!("Position: {:?}", pos);

        Ok(super::Mesh {
            mesh,
            tex,
            position: Vector3::from_array_storage(ArrayStorage([pos])),
            rotation: Vector3::from_array_storage(ArrayStorage([rot])),
            update,
        })
    }
}
