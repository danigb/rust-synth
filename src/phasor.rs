/**
 * @see https://pbat.ch/sndkit/phasor/
 */
pub struct Phasor {
    pub freq: f32,
    pub phase: f32,
    inv_sample_rate: f32,
}

impl Phasor {
    pub fn new(sample_rate: u32) -> Phasor {
        return Phasor {
            freq: 440.0,
            phase: 0.0,
            inv_sample_rate: 1.0 / sample_rate as f32,
        };
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }

    pub fn sample(&mut self) -> f32 {
        let output = self.phase;
        let increment = self.freq * self.inv_sample_rate;
        let mut phase = self.phase + increment;
        if phase >= 1.0 {
            phase -= 1.0;
        } else if phase < 0.0 {
            phase += 1.0
        }
        self.phase = phase;

        return output;
    }

    pub fn reset(&mut self, phase: f32) {
        if phase >= 0.0 {
            self.phase = phase;
        } else {
            self.phase = 0.0;
        }
    }
}
