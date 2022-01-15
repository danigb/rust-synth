mod generator;
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
    print!("{}", phasor.sample());

    let sin_wt = wavetable::sin(WAVETABLE_SIZE);
    let mut osc = WavetableOscillator::new(SAMPLE_RATE, sin_wt);
    osc.set_frequency(440.0);
    generator::play_gen(SAMPLE_RATE, 2.0, &mut osc)
}
