mod amp;
mod buffer;
mod envelope;
mod phasor;
mod signal;
mod wavetable;
mod wavetable_oscillator;

use amp::Amp;
use phasor::Phasor;
use signal::{Constant, Impulse, Signal};
use wavetable_oscillator::WavetableOscillator;

const SAMPLE_RATE: u32 = 44100;
const WAVETABLE_SIZE: usize = 1024;

fn main() {
    let mut phasor = Phasor::new(SAMPLE_RATE, Constant::new(440.0));
    let mut buffer = buffer::create_buffer(SAMPLE_RATE, 2.0);
    for _ in 0..buffer.capacity() {
        buffer.push(phasor.tick());
    }
    buffer::write_buffer("phasor.wav", SAMPLE_RATE, buffer);

    let sin_wt = wavetable::sin(WAVETABLE_SIZE);
    let mut osc = WavetableOscillator::new(SAMPLE_RATE, sin_wt, Constant::new(440.0));
    let mut buffer = buffer::create_buffer(SAMPLE_RATE, 2.0);
    for _ in 0..buffer.capacity() {
        buffer.push(osc.tick());
    }
    buffer::write_buffer("osc.wav", SAMPLE_RATE, buffer);

    let mut env = envelope::Envelope::new(SAMPLE_RATE, Impulse::new());
    env.set_attack(0.1);
    env.set_release(0.2);
    env.set_hold_in_seconds(0.1);

    let mut amp = Amp::new(osc, env);

    let mut buffer = buffer::create_buffer(SAMPLE_RATE, 2.0);
    for _ in 0..buffer.capacity() {
        buffer.push(amp.tick())
    }
    buffer::write_buffer("result.wav", SAMPLE_RATE, buffer);
}
