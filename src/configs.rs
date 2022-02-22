use confy;
use serde_derive::{
    Serialize,
    Deserialize
};

use home;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub version: u8,
    pub api_key: String,
    pub beep_pos: PathBuf,
}

pub fn load_config() -> Result<MyConfig, confy::ConfyError> {
    confy::load("timer")
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        let mut home = home::home_dir().unwrap();
        home.push(".config");
        home.push("timer");
        home.push("beep.mp3");
        Self {
            version: 0,
            api_key: "timer".into(),
            beep_pos: home
        }
    }
}
