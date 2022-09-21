use crate::signal::Signal;

/**
 * A RC Filter as explained in the VCV Rack book.
 *
 * See https://github.com/LOGUNIVPM/VCVBook/blob/master/ABC/src/RCFilter.hpp
 */
pub struct RcFilter<S: Signal> {
    source: S,

    inv_sample_rate: f32,
    prev: f32,
    coef: f32,
}

impl<S: Signal> RcFilter<S> {
    pub fn new(sample_rate: u32, coef: f32, source: S) -> Self {
        let inv_sample_rate = 1.0 / (sample_rate as f32);

        RcFilter {
            source,
            inv_sample_rate,
            prev: 0.0,
            coef,
        }
    }

    fn set_tau(&mut self, tau: f32) {
        self.coef = tau / (tau + self.inv_sample_rate);
    }

    fn set_cutoff(&mut self, cutoff: f32) {
        self.coef = 1.0 - cutoff * self.inv_sample_rate;
    }
}

impl<S: Signal> Signal for RcFilter<S> {
    fn tick(&mut self) -> f32 {
        let signal = self.source.tick();
        let next = self.coef * self.prev + (1.0 - self.coef) * signal;
        self.prev = next;
        next
    }
}
