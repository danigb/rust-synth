/**
 * A RC Filter as explained in the VCV Rack book.
 *
 * See https://github.com/LOGUNIVPM/VCVBook/blob/master/ABC/src/RCFilter.hpp
 */
pub struct RcFilter {
    inv_sample_rate: f32,
    prev: f32,
    coef: f32,
}

impl RcFilter {
    pub fn new(sample_rate: u32, coef: f32) -> Self {
        let inv_sample_rate = 1.0 / (sample_rate as f32);

        RcFilter {
            inv_sample_rate,
            prev: 0.0,
            coef,
        }
    }

    pub fn set_tau(&mut self, tau: f32) {
        self.coef = tau / (tau + self.inv_sample_rate);
    }

    fn set_cutoff(&mut self, cutoff: f32) {
        self.coef = 1.0 - cutoff * self.inv_sample_rate;
    }

    pub fn process(&mut self, signal: f32) -> f32 {
        let next = self.coef * self.prev + (1.0 - self.coef) * signal;
        self.prev = next;
        next
    }
}
