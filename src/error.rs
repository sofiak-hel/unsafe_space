#[derive(Debug)]
pub enum Error {
    RusqliteError(rusqlite::Error),
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match &self {
            Error::RusqliteError(err) => format!("SQLite error: {}", err),
            Error::IOError(err) => format!("IO error: {}", err),
        };
        write!(f, "{}", text)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        Error::RusqliteError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IOError(error)
    }
}
