use std::{cell::RefCell, rc::Rc};

use nalgebra::{ArrayStorage, Dyn, Vector3};
use rhai::{Dynamic, Engine, ParseError, Scope, AST};

use crate::fetch;

use super::{store::Mesh, Assets, LuaMsg, Store};
pub struct Control {
    engine: Engine,
    scope: Scope<'static, 8>,
    on_load: AST,
}

impl Control {
    pub async fn lua_msg(
        &mut self,
        msg: &LuaMsg,
        state: Rc<RefCell<Store>>,
        assets: Rc<RefCell<Assets>>,
    ) -> Result<(), String> {
        match msg {
            LuaMsg::Load(str) => {
                web_sys::console::log_1(&"Compiling...".into());
                match self.compile_on_load(str) {
                    Err(e) => return Err(format!("{:?}", e)),
                    Ok(()) => (),
                };

                web_sys::console::log_1(&"Compiled, Running...".into());
                let data = self.run_on_load().map_err(|e| e.to_string())?;

                self.load(state, assets, data)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    pub fn new() -> Self {
        let mut engine = Engine::new();
        engine.on_print(|x| web_sys::console::log_1(&x.into()));

        pub type RefMesh = Rc<RefCell<Mesh>>;

        engine
            .register_type_with_name::<RefMesh>("RefMesh")
            .register_get_set(
                "position",
                |p: &mut RefMesh| {
                    p.borrow()
                        .position
                        .iter()
                        .map(|&x| Dynamic::from_float(x as f64))
                        .collect::<Vec<Dynamic>>()
                },
                |p: &mut RefMesh, value: Vec<Dynamic>| {
                    let mut b = p.borrow_mut();
                    b.position[0] = value[0].as_float().unwrap() as f32;
                    b.position[1] = value[1].as_float().unwrap() as f32;
                    b.position[2] = value[2].as_float().unwrap() as f32;
                },
            )
            .register_get_set(
                "rotation",
                |p: &mut RefMesh| {
                    p.borrow()
                        .rotation
                        .iter()
                        .map(|&x| Dynamic::from_float(x as f64))
                        .collect::<Vec<Dynamic>>()
                },
                |p: &mut RefMesh, value: Vec<Dynamic>| {
                    let mut b = p.borrow_mut();
                    b.rotation[0] = value[0].as_float().unwrap() as f32;
                    b.rotation[1] = value[1].as_float().unwrap() as f32;
                    b.rotation[2] = value[2].as_float().unwrap() as f32;
                },
            );

        Control {
            on_load: engine.compile("40 + 2").unwrap(),
            scope: Scope::new(),
            engine,
        }
    }

    pub fn compile_on_load(&mut self, source: &str) -> Result<(), ParseError> {
        self.on_load = self.engine.compile(source)?;
        self.scope = Scope::new();

        Ok(())
    }

    pub fn run_on_load(&mut self) -> Result<Vec<Dynamic>, Box<rhai::EvalAltResult>> {
        web_sys::console::log_1(&"Running On Load".into());

        self.engine
            .eval_ast_with_scope(&mut self.scope, &self.on_load)
    }

    pub fn run_func(&mut self, func: &str, entity: Rc<RefCell<Mesh>>) {
        // ensure entity is borrowable, before we fail within the function
        entity.borrow_mut();

        self.engine
            .call_fn::<Dynamic>(&mut self.scope, &self.on_load, func, (entity,))
            .unwrap();
    }

    pub fn bool_or(e: &rhai::Map, s: &str, or: bool) -> bool {
        match e.get(s) {
            Some(v) => v.as_bool().unwrap(),
            None => or,
        }
    }
    pub fn f32_or(e: &rhai::Map, s: &str, or: f32) -> f32 {
        match e.get(s) {
            Some(v) => Self::to_f32(v).unwrap(),
            None => or,
        }
    }

    pub fn to_f32(d: &Dynamic) -> Result<f32, &'static str> {
        match d.as_float() {
            Ok(f) => Ok(f as f32),
            Err(e) => match d.as_int() {
                Ok(i) => Ok(i as f32),
                Err(e) => Err("Non-numeric integer type found"),
            },
        }
    }

    pub fn to_vec3(d: &Dynamic) -> Result<[f32; 3], &'static str> {
        let x = d.clone().into_array()?;

        match x.len() {
            3 => Ok([
                Self::to_f32(&x[0])?,
                Self::to_f32(&x[1])?,
                Self::to_f32(&x[2])?,
            ]),
            _ => Err("Incorrect length of vector"),
        }
    }

    pub async fn load(
        &mut self,
        state: Rc<RefCell<Store>>,
        assets: Rc<RefCell<Assets>>,
        data: Vec<Dynamic>,
    ) -> Result<(), &'static str> {
        // Apply data put into data table

        //web_sys::console::log_1(&format!("{:?}", data).into());

        let default_pos = [0f32, 0f32, 0f32];

        state.borrow_mut().state.entities.clear();

        for dyn_entity in &data {
            let entity = dyn_entity.clone_cast::<rhai::Map>();

            let e = match entity["type"].clone().into_string()?.as_str() {
                "mesh" => {
                    let mesh = entity["mesh"].clone().into_string().unwrap().to_owned();

                    // Load the mesh if it doesnt exist already
                    if assets.borrow_mut().get_gltf(&mesh).is_none() {
                        let data = fetch::fetch(&mesh).await.unwrap();

                        assets
                            .borrow_mut()
                            .load_gltf(mesh.clone(), &data[..])
                            .unwrap();
                    }

                    let pos = match entity.get("position") {
                        Some(d) => Self::to_vec3(d)?,
                        None => default_pos.clone(),
                    };
                    let rot = match entity.get("rotation") {
                        Some(d) => Self::to_vec3(d)?,
                        None => default_pos.clone(),
                    };
                    let update = entity
                        .get("update")
                        .map(|f| f.clone().cast::<rhai::FnPtr>().fn_name().to_owned());

                    web_sys::console::log_1(&format!("Position: {:?}", pos).into());

                    super::Entity::EntMesh(Rc::new(RefCell::new(super::Mesh {
                        name: mesh,
                        position: Vector3::from_array_storage(ArrayStorage([pos])),
                        rotation: Vector3::from_array_storage(ArrayStorage([rot])),
                        update,
                    })))
                }
                "water" => super::Entity::EntWater(crate::app::store::water::Water {
                    reflectivity: Self::f32_or(&entity, "reflectivity", 0.5),
                    fresnel_strength: Self::f32_or(&entity, "fresnel", 0.5),
                    wave_speed: Self::f32_or(&entity, "wave_speed", 0.5),
                    use_refraction: Self::bool_or(&entity, "use_refraction", true),
                    use_reflection: Self::bool_or(&entity, "use_reflection", true),
                }),
                _ => return Err("Unknown Entity Type"),
            };
            web_sys::console::log_1(&format!("{:?}", e).into());

            state.borrow_mut().state.entities.push(Box::new(e))
        }

        web_sys::console::log_1(&format!("{:?}", state.borrow_mut().state.entities).into());

        Ok(())
    }
}
