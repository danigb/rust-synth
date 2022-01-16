use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use std::path;

pub fn create_buffer(sample_rate: u32, duration_in_seconds: f32) -> Vec<f32> {
    let length = (sample_rate as f32 * duration_in_seconds) as usize;
    let buffer: Vec<f32> = Vec::with_capacity(length);
    return buffer;
}

pub fn play_buffer(sample_rate: u32, buffer: Vec<f32>) {
    let source = SamplesBuffer::new(1, sample_rate, buffer);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}

pub fn write_buffer<P: AsRef<path::Path>>(filename: P, sample_rate: u32, buffer: Vec<f32>) {
    let spec = hound::WavSpec {
        sample_rate,
        channels: 1,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();

    for value in buffer.iter() {
        writer.write_sample(*value).unwrap();
    }
    writer.finalize().unwrap();
}
