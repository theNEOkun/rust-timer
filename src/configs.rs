use confy;
use serde_derive::{
    Serialize,
    Deserialize
};

use home;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub beep_pos: PathBuf,
    pub digit_pos: String,
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
        let mut digits = home::home_dir().unwrap().into_os_string().into_string().expect("");
        digits += "/.config/timer/digits/";
        Self {
            beep_pos: home,
            digit_pos: digits,
        }
    }
}
