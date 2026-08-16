use crate::config::Config;

pub struct Config_Menu {
    configs: Vec<Config>
}

impl Config_Menu {
    pub fn new() -> Config_Menu {
        Self {
            configs: Vec::new()
        }
    }
}
