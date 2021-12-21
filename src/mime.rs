use crate::Result;
use std::collections::HashMap;
use std::path::Path;

static DEFAULT_MIMETYPES: &str = include_str!("mime.types");

#[derive(Debug, Clone)]
pub struct MimeTypes {
    mimetypes: HashMap<String, String>,
    #[allow(dead_code)]
    extensions: HashMap<String, Vec<String>>,
}

impl MimeTypes {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<MimeTypes> {
        MimeTypes::from_text(std::fs::read_to_string(path)?)
    }

    pub fn from_text<T: Into<String>>(text: T) -> Result<MimeTypes> {
        let text = text.into();
        let mut lines = text.lines();
        let mut mimetypes = HashMap::new();
        let mut extensions = HashMap::new();

        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.starts_with("#") {
                continue;
            } else {
                let mut parts = line.split_ascii_whitespace().map(|p| p.to_owned());
                if let Some(mimetype) = parts.next() {
                    extensions.insert(mimetype.clone(), parts.clone().collect());
                    while let Some(part) = parts.next() {
                        mimetypes.insert(part.to_owned(), mimetype.clone());
                    }
                }
            }
        }

        Ok(MimeTypes {
            mimetypes,
            extensions,
        })
    }

    #[allow(dead_code)]
    pub fn extensions<T: Into<String>>(&self, mimetype: T) -> Option<&Vec<String>> {
        self.extensions.get(&mimetype.into())
    }

    pub fn mimetype<T: Into<String>>(&self, extension: T) -> Option<&String> {
        self.mimetypes.get(&extension.into())
    }
}

impl Default for MimeTypes {
    fn default() -> MimeTypes {
        MimeTypes::from_text(DEFAULT_MIMETYPES).unwrap()
    }
}
