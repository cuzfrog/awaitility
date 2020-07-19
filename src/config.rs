use std::time::Duration;

static DEFAULT_INTERVAL: Duration = Duration::from_millis(50);

pub struct Config<'a> {
    pub interval: Duration,
    description: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn default() -> Config<'static> {
        Config {
            interval: DEFAULT_INTERVAL,
            description: None,
        }
    }

    #[inline]
    pub fn fail(&self, basic_desc: &str) {
        let desc = self.description.unwrap_or("");
        panic!("{} {}", basic_desc, desc);
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }

    pub fn set_description<'b: 'a>(&mut self, desc: &'b str) {
        self.description = Some(desc);
    }
}
