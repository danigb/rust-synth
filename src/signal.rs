pub trait Signal {
    fn tick(&mut self) -> f32;
}

pub struct Impulse {
    first: bool,
}

impl Impulse {
    pub fn new() -> Self {
        Impulse { first: true }
    }
}

impl Signal for Impulse {
    fn tick(&mut self) -> f32 {
        if self.first {
            self.first = false;
            1.0
        } else {
            0.0
        }
    }
}

pub struct Constant {
    value: f32,
}

impl Constant {
    pub fn new(value: f32) -> Self {
        Constant { value }
    }
}

impl Signal for Constant {
    fn tick(&mut self) -> f32 {
        self.value
    }
}
