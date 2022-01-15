use hound;
use std::f32::consts::PI;

/**
 * Code from: https://docs.rs/hound/latest/hound/index.html
 * And adapted to follow Basic Synth 5.2
 */
fn main() {
    let sample_rate = 44100;
    let spec = hound::WavSpec {
        sample_rate,
        channels: 1,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    let angular_freq = 2 as f32 * PI / sample_rate as f32;
    let duration_in_seconds = 2.0;
    let total_samples = (sample_rate as f32 * duration_in_seconds) as u32;
    let freq = 440.0;
    for n in 0..total_samples {
        let sample = (n as f32 * freq * angular_freq).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}
