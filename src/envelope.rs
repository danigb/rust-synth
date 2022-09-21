use crate::signal::Signal;

const EPSILON: f32 = 5e-8;

enum Mode {
    NONE,
    ATTACK,
    HOLD,
    RELEASE,
}

pub struct Envelope<T: Signal> {
    // Params
    trigger: T,

    // Internal state
    pub sample_rate: u32,
    // The timer is stored in a normalized range [0,1],
    // which allows the hold time to be adjustable while it is in hold mode.
    timer: f32,
    timer_inc: f32,
    atk_env: f32,
    rel_env: f32,
    mode: Mode,
    // the output from the previous tick
    prev: f32,
}

impl<T: Signal> Envelope<T> {
    pub fn new(sample_rate: u32, attack: f32, release: f32, hold: f32, trigger: T) -> Self {
        let mut env = Envelope {
            trigger,

            sample_rate,
            timer: 0.0,
            timer_inc: 0.0,
            atk_env: 0.0,
            rel_env: 0.0,
            mode: Mode::NONE,
            prev: 0.0,
        };
        env.set_attack(attack);
        env.set_release(release);
        env.set_hold_in_seconds(hold);
        env
    }

    fn set_attack(&mut self, attack: f32) {
        self.atk_env = (-1.0 / (attack * self.sample_rate as f32)).exp()
    }

    fn set_release(&mut self, release: f32) {
        self.rel_env = (-1.0 / (release * self.sample_rate as f32)).exp()
    }

    fn set_hold_in_seconds(&mut self, hold: f32) {
        if hold > 0.0 {
            self.timer_inc = 1.0 / (hold * self.sample_rate as f32)
        } else {
            self.timer_inc = 1.0
        }
    }
}

impl<T: Signal> Signal for Envelope<T> {
    fn tick(&mut self) -> f32 {
        let mut out = 0.0;

        let trigger = self.trigger.tick();
        if trigger != 0.0 {
            self.mode = Mode::ATTACK;
        }

        match self.mode {
            Mode::ATTACK => {
                out = self.atk_env * self.prev + (1.0 - self.atk_env);

                if (out - self.prev) <= EPSILON {
                    println!("ATTACK TO HOLD: {:?}", out);
                    self.mode = Mode::HOLD;
                    self.timer = 0.0
                }
            }
            Mode::HOLD => {
                out = self.prev;
                self.timer += self.timer_inc;

                if self.timer >= 1.0 {
                    println!("HOLD TO RELEASE: {:?}", out);
                    self.mode = Mode::RELEASE;
                }
            }
            Mode::RELEASE => {
                out = self.rel_env * self.prev;

                if out <= EPSILON {
                    println!("RELEASE TO NONE: {:?}", out);
                    self.mode = Mode::NONE;
                }
            }
            Mode::NONE => {}
        }

        self.prev = out;
        return out;
    }
}
