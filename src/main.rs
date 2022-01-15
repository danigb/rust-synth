use std::f32::consts::PI;
mod wavetable_oscillator;
use hound;
use rodio::{source::Source, OutputStream};

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

pub fn play_oscillator() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        wave_table.push((2.0 * PI * n as f32 / wave_table_size as f32).sin());
    }

    let mut oscillator = wavetable_oscillator::WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}
