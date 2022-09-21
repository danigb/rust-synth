mod amp;
mod buffer;
mod envelope;
mod phasor;
mod rc_filter;
mod signal;
mod wavetable;
mod wavetable_oscillator;

use amp::Amp;
use buffer::create_wav_file;
use envelope::Envelope;
use phasor::Phasor;
use signal::{Constant, Impulse, Scale, Signal};
use wavetable_oscillator::WavetableOscillator;

const SAMPLE_RATE: u32 = 44100;
const WAVETABLE_SIZE: usize = 1024;

fn main() {
    let phasor = Phasor::new(SAMPLE_RATE, Constant::new(440.0));
    create_wav_file("phasor.wav", SAMPLE_RATE, 2.0, phasor);

    let sin_wavetable = wavetable::sin(WAVETABLE_SIZE);
    let filter_freq = Scale::linear(
        200.0,
        800.0,
        Envelope::new(SAMPLE_RATE, 0.3, 0.2, 0.0, Impulse::new()),
    );
    let osc = WavetableOscillator::new(SAMPLE_RATE, sin_wavetable, filter_freq);
    let amp_env = Envelope::new(SAMPLE_RATE, 0.1, 0.2, 0.1, Impulse::new());
    let amp = Amp::new(osc, amp_env);

    let output = amp;
    create_wav_file("result.wav", SAMPLE_RATE, 2.0, output);
}
