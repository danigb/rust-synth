use crate::generator::Generator;

pub struct WavetableOscillator {
    pub sample_rate: u32,
    pub phase: f32,
    phase_increment: f32,
    wave_table: Vec<f32>,
    table_size: usize,
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

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        let table_size = wave_table.len();

        return WavetableOscillator {
            sample_rate,
            wave_table,
            phase: 0.0,
            phase_increment: 0.0,
            table_size,
        };
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.phase_increment = frequency / self.sample_rate as f32;
    }

    pub fn tick(&mut self) -> f32 {
        let sample = self.get_interpolated_value(self.phase);
        self.phase = clamp_phase(self.phase + self.phase_increment);
        return sample;
    }

    pub fn tick_with_external_phase(&self, phase: f32) -> f32 {
        let clamped_phase = clamp_phase(phase);

        let sample = self.get_interpolated_value(clamped_phase);
        return sample;
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

impl Generator for WavetableOscillator {
    fn tick(&mut self) -> f32 {
        return self.tick();
    }
}
