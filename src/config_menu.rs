use crate::config::Config;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use ratatui::widgets::ListState;
use std::path::Path;
use std::time::Duration;
use serde_json::{from_reader, Error};
use std::fs::*;
use std::io::BufReader;

const CONFIGS_FILE_PATH: &str = "configs.json";
pub struct ConfigMenu {
    configs: Vec<Config>,
    pub list_state: ListState
}

impl ConfigMenu {
    pub fn new() -> ConfigMenu {
        Self {
            configs: Vec::new(),
            list_state: ListState::default()
        }
    }

    pub fn read_configs_from_file(&mut self) -> Result<()> {
        let path = Path::new(CONFIGS_FILE_PATH);
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            match from_reader(reader) {
                Ok(json) => self.configs = json,
                Err(_) => {}
            }
        }
        Ok(())
    }
    pub fn in_string_format(&self) -> Vec<String> {
        let strings: Vec<String> = self.configs.iter()
            .map(|config| config.to_string()).collect();
        strings
    }

    pub fn select_config(&self) -> Option<Config> {
        match self.list_state.selected() {
            Some(ind) => Some(self.configs[ind]),
            None => None
        }
    }
    pub fn remove_task(&mut self, i: Option<usize>) {
        if let Some(i) = i {
            if self.configs.len() == 1 {
                self.list_state.select(None);
            }
            self.configs.remove(i);
        }
    }
    pub fn remove_config(&mut self) {
        if self.configs.is_empty() {return}
        if let Some(i) = self.list_state.selected() {
            self.configs.remove(i);
            //Ensures that after an item is deleted, the list state is not out of bounds
            if self.configs.is_empty() {
                self.list_state.select(None);
            } else {
                self.list_state.select(Some(i.clamp(0, self.configs.len() - 1)));
            }
        }
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }
    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }
    

    //TODO - Add a way to inform user if file saving has not taken place.
    //This may happen if the configs.json file can't be found
    pub fn save_configs(&self) -> Result<()> {
        match self.serialize() {
            Ok(json) => match write_list_to_file(&json, CONFIGS_FILE_PATH) {
                Ok(()) => {},
                Err(_) => {}
            },
            Err(_) => {}
        }
        Ok(())
    }

    fn serialize(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(&self.configs)
    }
}
fn write_list_to_file(data: &str, file_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        write(path, data)?;
        Ok(())
}

impl Default for ConfigMenu {
    fn default() -> ConfigMenu {
        let configs = vec![Config::new(Duration::new(4, 0), Duration::new(5, 0), 5).unwrap()];

        Self {
            configs: configs,
            list_state: ListState::default()
        }
    }
}
