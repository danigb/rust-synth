use crate::signal::Signal;

/**
 * @see https://pbat.ch/sndkit/phasor/
 */
pub struct Phasor<F: Signal> {
    frequency: F,
    phase: f32,
    inv_sample_rate: f32,
}

impl<F: Signal> Phasor<F> {
    pub fn new(sample_rate: u32, frequency: F) -> Self {
        let inv_sample_rate = 1.0 / (sample_rate as f32);
        let phase = 0.0;

        Phasor {
            frequency,
            phase,
            inv_sample_rate,
        }
    }

    pub fn reset(&mut self, phase: f32) {
        if phase >= 0.0 {
            self.phase = phase;
        } else {
            self.phase = 0.0;
        }
    }
}

impl<F: Signal> Signal for Phasor<F> {
    fn tick(&mut self) -> f32 {
        let freq = self.frequency.tick();
        let output = self.phase;
        let increment = freq * self.inv_sample_rate;
        let mut phase = self.phase + increment;
        if phase >= 1.0 {
            phase -= 1.0;
        } else if phase < 0.0 {
            phase += 1.0
        }
        self.phase = phase;

        return output;
    }
}
