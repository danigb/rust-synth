mod buffer;
mod env;
mod osc;
mod phasor;
mod wavetable;

use osc::WavetableOscillator;
use phasor::Phasor;

const SAMPLE_RATE: u32 = 44100;
const WAVETABLE_SIZE: usize = 1024;

fn main() {
    let mut phasor = Phasor::new(SAMPLE_RATE);
    phasor.set_frequency(440.0);
    let mut buffer = buffer::create_buffer(SAMPLE_RATE, 2.0);
    for _ in 0..buffer.capacity() {
        buffer.push(phasor.tick());
    }
    buffer::write_buffer("phasor.wav", SAMPLE_RATE, buffer);

    let sin_wt = wavetable::sin(WAVETABLE_SIZE);
    let mut osc = WavetableOscillator::new(SAMPLE_RATE, sin_wt);
    osc.set_frequency(440.0);
    let mut buffer = buffer::create_buffer(SAMPLE_RATE, 2.0);
    for _ in 0..buffer.capacity() {
        buffer.push(osc.tick());
    }
    buffer::write_buffer("osc.wav", SAMPLE_RATE, buffer);

    let mut env = env::Envelope::new(SAMPLE_RATE);
    env.set_attack(0.1);
    env.set_release(0.2);
    env.set_hold_in_seconds(0.1);

    let mut buffer = buffer::create_buffer(SAMPLE_RATE, 2.0);
    buffer.push(env.tick(1.0));
    for _ in 1..buffer.capacity() {
        buffer.push(env.tick(0.0))
    }
    buffer::write_buffer("env.wav", SAMPLE_RATE, buffer);
}
