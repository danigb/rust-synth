use crate::signal::Signal;

pub struct Amp<S: Signal, G: Signal> {
    gain: G,
    source: S,
}
impl<S: Signal, G: Signal> Amp<S, G> {
    pub fn new(source: S, gain: G) -> Self {
        Amp { source, gain }
    }
}

impl<S: Signal, G: Signal> Signal for Amp<S, G> {
    fn tick(&mut self) -> f32 {
        let gain = self.gain.tick();
        let source = self.source.tick();
        return gain * source;
    }
}
