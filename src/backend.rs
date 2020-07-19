use crate::error::Timeout;
use std::time::Duration;

const DEFAULT_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Clone)]
enum FailStrategy {
    Panic,
    Return,
}

#[derive(Clone)]
pub struct Backend<'a> {
    pub interval: Duration,
    description: Option<&'a str>,
    fail_strategy: FailStrategy,
    pub result: Result<(), Timeout>,
}

impl<'a> Backend<'a> {
    pub const fn default() -> Backend<'static> {
        Backend {
            interval: DEFAULT_INTERVAL,
            description: None,
            fail_strategy: FailStrategy::Panic,
            result: Ok(()),
        }
    }

    #[inline]
    pub fn fail(&mut self, basic_desc: &str) {
        let desc = self.description.unwrap_or("");
        match self.fail_strategy {
            FailStrategy::Panic => panic!("{} {}", basic_desc, desc),
            FailStrategy::Return => self.result = Err(Timeout::new(format!("{} {}", basic_desc, desc))),
        };
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }

    pub fn set_description<'b: 'a>(&mut self, desc: &'b str) {
        self.description = Some(desc);
    }

    pub fn set_return(&mut self) {
        self.fail_strategy = FailStrategy::Return;
    }
}

#[cfg(test)]
mod backend_test {
    use super::Backend;

    #[test]
    #[should_panic]
    fn fail_test() {
        let mut backend = Backend::default();
        backend.fail("");
    }
}
