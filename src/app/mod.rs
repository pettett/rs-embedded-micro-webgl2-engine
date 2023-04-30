use std::cell::RefCell;
use std::rc::Rc;

mod control;
pub mod ray;
pub mod render;
pub use self::control::*;

pub mod store;
pub use self::store::*;

mod assets;
pub use self::assets::*;

/// Used to instantiate our application
pub struct App {
    pub assets: Rc<RefCell<Assets>>,
    pub store: Rc<RefCell<Store>>,
    pub control: Rc<RefCell<Control>>,
}

impl App {
    /// Create a new instance of our WebGL Water application
    pub fn new() -> App {
        let assets = Rc::new(RefCell::new(Assets::new()));
        App {
            control: Rc::new(RefCell::new(Control::new(assets.clone()))),
            assets,
            store: Rc::new(RefCell::new(Store::new())),
        }
    }
}
