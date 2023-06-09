use std::{cell::RefCell, rc::Rc};

use rhai::{Dynamic, Engine, ParseError, Scope, AST};

pub mod from_rhai;
mod mesh_from_rhai;

use crate::app::render::material::mat::Uniform;

use self::from_rhai::FromRhai;

use super::{
    render::material::mat::Mat,
    store::{entity::Entity, Mesh},
    Assets, LuaMsg, Store,
};
pub struct Control {
    engine: Engine,
    scope: Scope<'static, 8>,
    on_load: AST,
}

pub fn bool_or(e: &rhai::Map, s: &str, or: bool) -> bool {
    match e.get(s) {
        Some(v) => v.as_bool().unwrap(),
        None => or,
    }
}
pub fn f32_or(e: &rhai::Map, s: &str, or: f32) -> f32 {
    match e.get(s) {
        Some(v) => to_f32(v).unwrap(),
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
        3 => Ok([to_f32(&x[0])?, to_f32(&x[1])?, to_f32(&x[2])?]),
        _ => Err("Incorrect length of vector"),
    }
}

impl Control {
    pub fn lua_msg(
        &mut self,
        msg: &LuaMsg,
        state: Rc<RefCell<Store>>,
        assets: Rc<RefCell<Assets>>,
    ) -> Result<(), String> {
        match msg {
            LuaMsg::Load(str) => {
                log::info!("Compiling...");
                match self.compile_on_load(str) {
                    Err(e) => return Err(format!("{:?}", e)),
                    Ok(()) => (),
                };

                log::info!("Compiled, Running...");
                let data = self.run_on_load().map_err(|e| e.to_string())?;

                self.load(state, assets, data).map_err(|e| e.to_string())
            }
        }
    }

    pub fn new(assets: Rc<RefCell<Assets>>) -> Self {
        let mut engine = Engine::new();
        engine.on_print(|x| log::info!("{}", x));

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

        engine.register_fn("tex", move |name: String| {
            Uniform::Tex(assets.borrow_mut().require_texture(name))
        });

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
        log::info!("Running On Load");

        self.engine
            .eval_ast_with_scope(&mut self.scope, &self.on_load)
    }

    pub fn run_func(&mut self, func: &str, entity: Rc<RefCell<dyn Entity>>) -> Dynamic {
        // ensure entity is borrowable, before we fail within the function
        entity.borrow_mut();

        self.engine
            .call_fn(&mut self.scope, &self.on_load, func, (entity,))
            .unwrap()
    }

    pub fn load(
        &mut self,
        state: Rc<RefCell<Store>>,
        assets: Rc<RefCell<Assets>>,
        data: Vec<Dynamic>,
    ) -> Result<(), &'static str> {
        // Apply data put into data table

        //web_sys::console::log_1(&format!("{:?}", data).into());

        let default_pos = [0f32, 0f32, 0f32];

        state.borrow_mut().state.entities.clear();

        for dyn_entity in data {
            let entity = dyn_entity.cast::<rhai::Map>();

            match entity["type"].clone().into_string()?.as_str() {
                "mesh" => {
                    let m = super::Mesh::try_from_rhai(entity, &mut assets.borrow_mut()).unwrap();

                    let e: Rc<RefCell<dyn Entity>> = Rc::new(RefCell::new(m));
                    state.borrow_mut().state.entities.push(e)
                }
                "mat" => {
                    let name = entity["name"].clone().into_string()?;
                    let mat = Mat::try_from_rhai(entity, &mut assets.borrow_mut()).unwrap();

                    assets.borrow_mut().insert_material(name, mat)
                }
                "water" => {
                    let d = assets
                        .borrow_mut()
                        .require_texture("assets/textures/dudvmap.png".to_owned());

                    let n = assets
                        .borrow_mut()
                        .require_texture("assets/textures/normalmap.png".to_owned());

                    let e: Rc<RefCell<dyn Entity>> =
                        Rc::new(RefCell::new(crate::app::store::water::Water {
                            dudv: d,
                            normal: n,
                            reflectivity: f32_or(&entity, "reflectivity", 0.5),
                            fresnel_strength: f32_or(&entity, "fresnel", 0.5),
                            wave_speed: f32_or(&entity, "wave_speed", 0.5),
                            use_refraction: bool_or(&entity, "use_refraction", true),
                            use_reflection: bool_or(&entity, "use_reflection", true),
                        }));
                    state.borrow_mut().state.entities.push(e)
                }
                _ => return Err("Unknown Entity Type"),
            };
            //log::info!("{:?}", e);
        }

        //log::info!("{:?}", state.borrow_mut().state.entities);

        Ok(())
    }
}
