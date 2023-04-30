#[derive(Default)]
pub struct Keyboard {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
}

#[derive(FromPrimitive, Debug, Copy, Clone)]
pub enum KeyCode {
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
}

impl Keyboard {
    pub fn get_pressed(&self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::W => self.w,
            KeyCode::A => self.a,
            KeyCode::S => self.s,
            KeyCode::D => self.d,
            _ => false,
        }
    }

    pub fn set_pressed(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
            KeyCode::W => self.w = pressed,
            KeyCode::A => self.a = pressed,
            KeyCode::S => self.s = pressed,
            KeyCode::D => self.d = pressed,
            _ => (),
        }
    }
}
