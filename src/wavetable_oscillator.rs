use crate::signal::Signal;

pub struct WavetableOscillator<F: Signal> {
    frequency: F,
    inv_sample_rate: f32,
    pub phase: f32,
    table_size: usize,
    wave_table: Vec<f32>,
}

fn clamp_phase(phase: f32) -> f32 {
    let mut clamped = phase;
    while clamped >= 1.0 {
        clamped -= 1.0;
    }
    while clamped < 0.0 {
        clamped += 1.0;
    }
    return clamped;
}

impl<F: Signal> WavetableOscillator<F> {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>, frequency: F) -> Self {
        let table_size = wave_table.len();
        let inv_sample_rate = 1.0 / (sample_rate as f32);

        return WavetableOscillator {
            frequency,
            inv_sample_rate,
            wave_table,
            phase: 0.0,
            table_size,
        };
    }

    fn get_interpolated_value(&self, phase: f32) -> f32 {
        let index = phase * self.table_size as f32;
        let truncated_index = index as usize;
        let next_index = (truncated_index + 1) % self.table_size;
        let next_index_weight = index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl<F: Signal> Signal for WavetableOscillator<F> {
    fn tick(&mut self) -> f32 {
        let freq = self.frequency.tick();
        let phase_increment = freq * self.inv_sample_rate;
        let sample = self.get_interpolated_value(self.phase);
        self.phase = clamp_phase(self.phase + phase_increment);
        sample
    }
}
