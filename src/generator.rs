use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};

pub trait Generator {
    fn tick(&mut self) -> f32;
}

pub fn play_gen(sample_rate: u32, duration_in_seconds: f32, gen: &mut impl Generator) {
    let length = (sample_rate as f32 * duration_in_seconds) as usize;
    let mut buffer: Vec<f32> = Vec::with_capacity(length);
    for n in 0..length {
        buffer.push(gen.tick());
    }

    let source = SamplesBuffer::new(1, sample_rate, buffer);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
