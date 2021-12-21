use std::{fs::File, io::Read, path::PathBuf};

use serde::Deserialize;

use crate::Result;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub force_recreate_db: bool,
    pub session_exp: i64,
    pub static_path: PathBuf,
    pub log_level: String,
    pub logging_template: String,
    pub mimetypes_path: PathBuf,
}

impl Config {
    pub fn from_file<T: Into<PathBuf>>(path: T) -> Result<Config> {
        let mut text = String::new();
        File::open(path.into())?.read_to_string(&mut text)?;
        Ok(toml::from_str(&text)?)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: "0.0.0.0".to_owned(),
            port: 8080,
            force_recreate_db: false,
            session_exp: 60,
            static_path: PathBuf::from("./static").canonicalize().unwrap(),
            log_level: "INFO".to_owned(),
            logging_template: "%r %s %a".to_owned(),
            mimetypes_path: PathBuf::from("/etc/mime.types").canonicalize().unwrap(),
        }
    }
}
