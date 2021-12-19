use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub force_recreate_db: bool,
    pub session_exp: u64,
    pub static_path: PathBuf,
    pub log_level: String,
    pub logging_template: String,
    pub mimetypes_path: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: "127.0.0.1".to_owned(),
            port: 8080,
            force_recreate_db: false,
            session_exp: 3600,
            static_path: PathBuf::from("./static").canonicalize().unwrap(),
            log_level: "INFO".to_owned(),
            logging_template: "%r %s %a".to_owned(),
            mimetypes_path: PathBuf::from("/etc/mime.types").canonicalize().unwrap(),
        }
    }
}
