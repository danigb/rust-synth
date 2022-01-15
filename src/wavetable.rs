use std::f32::consts::PI;

pub fn sin(size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(size);
    for n in 0..size {
        wave_table.push((2.0 * PI * n as f32 / size as f32).sin());
    }
    return wave_table;
}
