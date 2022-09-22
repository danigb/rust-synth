use crate::signal::Signal;

pub struct Gate {
    current_sample: usize,
    start_sample: usize,
    stop_sample: usize,
}

impl Gate {
    pub fn new(sample_rate: u32, delay_time: f64, duration_time: f64) -> Self {
        let start_sample = ((sample_rate as f64) * delay_time).round() as usize;
        let duration_in_samples = ((sample_rate as f64) * duration_time).round() as usize;
        let stop_sample = start_sample + duration_in_samples;

        Gate {
            current_sample: 0,
            start_sample,
            stop_sample,
        }
    }
}

impl Signal for Gate {
    fn tick(&mut self) -> f32 {
        let value =
            if self.current_sample < self.start_sample || self.current_sample > self.stop_sample {
                0.0
            } else {
                1.0
            };
        self.current_sample += 1;
        value
    }
}
