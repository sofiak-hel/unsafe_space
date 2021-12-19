#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub session_exp: u64,
    pub log_level: String,
    pub logging_template: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: "127.0.0.1".to_owned(),
            port: 8080,
            session_exp: 60,
            log_level: "INFO".to_owned(),
            logging_template: "%r %s %a".to_owned(),
        }
    }
}
