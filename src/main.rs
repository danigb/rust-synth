mod phasor;
use phasor::Phasor;

const SAMPLE_RATE: u32 = 44100;

fn main() {
    let mut phasor = Phasor::new(SAMPLE_RATE);
    phasor.set_frequency(440.0);
    print!("{}", phasor.sample());
}
