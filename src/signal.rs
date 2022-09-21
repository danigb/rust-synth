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

pub struct Scale<S: Signal> {
    source: S,

    // internal state
    min: f32,
    mul: f32,
}

impl<S: Signal> Scale<S> {
    pub fn linear(min: f32, max: f32, source: S) -> Self {
        let mul = max - min;
        Scale { source, min, mul }
    }
}

impl<S: Signal> Signal for Scale<S> {
    fn tick(&mut self) -> f32 {
        let source = self.source.tick();
        source * self.mul + self.min
    }
}
