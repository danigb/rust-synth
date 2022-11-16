pub struct BoolTrigger {
    current: bool,
}

impl BoolTrigger {
    pub fn new() -> Self {
        BoolTrigger { current: false }
    }

    pub fn process(&mut self, state: bool) -> bool {
        let triggered = state && !self.current;

        self.current = state;
        triggered
    }
}

#[cfg(test)]
mod tests {
    use super::BoolTrigger;

    #[test]
    fn test_trigger() {
        let mut trigger = BoolTrigger::new();
        assert_eq!(trigger.process(false), false);
        assert_eq!(trigger.process(true), true);
        assert_eq!(trigger.process(true), false);
        assert_eq!(trigger.process(false), false);
        assert_eq!(trigger.process(true), true);
    }
}
