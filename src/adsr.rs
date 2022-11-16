use crate::{
    rc_filter::RcFilter,
    signal::{Param, Signal},
    triggers::BoolTrigger,
};

const EPSILON: f32 = 1e-9;

/**
 * An envelope generator as explained in VCV Rack book
 *
 * See https://github.com/LOGUNIVPM/VCVBook/blob/master/ABC/src/AExpADSR.cpp
 */

pub struct Adsr<A: Signal, D: Signal, S: Signal, R: Signal, G: Signal> {
    attack: A,
    decay: D,
    sustain: S,
    release: R,
    gate: G,

    // private state
    envelope: f32,
    is_running: bool,
    is_attack: bool,
    trigger: BoolTrigger,
    rc_filter: RcFilter,
}

impl<A: Signal, D: Signal, S: Signal, R: Signal, G: Signal> Adsr<A, D, S, R, G> {
    pub fn new(sample_rate: u32, attack: A, decay: D, sustain: S, release: R, gate: G) -> Self {
        Adsr {
            attack,
            decay,
            sustain,
            release,
            gate,

            envelope: 0.0,
            is_running: false,
            is_attack: false,
            trigger: BoolTrigger::new(),
            rc_filter: RcFilter::new(sample_rate, 0.999),
        }
    }
}

pub fn adsr<G: Signal>(sample_rate: u32, gate: G) -> Adsr<Param, Param, Param, Param, G> {
    let attack = Param::new(0.1);
    let decay = Param::new(0.1);
    let sustain = Param::new(0.5);
    let release = Param::new(0.1);
    Adsr::new(sample_rate, attack, decay, sustain, release, gate)
}

impl<A: Signal, D: Signal, S: Signal, R: Signal, G: Signal> Signal for Adsr<A, D, S, R, G> {
    fn tick(&mut self) -> f32 {
        let attack = self.attack.tick();
        let decay = self.decay.tick();
        let sustain = self.sustain.tick();
        let release = self.release.tick();

        let is_gate = self.gate.tick() >= 1.0;
        if self.trigger.process(is_gate) {
            self.is_running = true;
            self.is_attack = true;
        }

        if self.is_running {
            if is_gate {
                if self.is_attack {
                    // attack
                    let a_tau = attack.clamp(EPSILON, 5.0);
                    self.rc_filter.set_tau(a_tau);
                    self.envelope = self.rc_filter.process(1.0);
                    if self.envelope >= 1.0 - 0.001 {
                        self.is_attack = false;
                    }
                } else {
                    // decay
                    let d_tau = decay.clamp(EPSILON, 5.0);
                    self.rc_filter.set_tau(d_tau);
                    if self.envelope <= sustain + 0.001 {
                        self.envelope = sustain;
                    } else {
                        self.envelope = self.rc_filter.process(sustain);
                    }
                }
            } else {
                // release
                let r_tau = release.clamp(EPSILON, 5.0);
                self.rc_filter.set_tau(r_tau);
                self.envelope = self.rc_filter.process(0.0);
                if self.envelope < 0.001 {
                    self.is_running = false;
                }
            }
        } else {
            // not running
            self.envelope = 0.0;
        };

        self.envelope
    }
}
