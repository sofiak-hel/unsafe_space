use crate::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MimeTypes {
    mimetypes: HashMap<String, String>,
    extensions: HashMap<String, Vec<String>>,
}

impl MimeTypes {
    pub fn from<T: AsRef<Path>>(path: T) -> Result<MimeTypes> {
        let mut lines = BufReader::new(File::open(path)?).lines();
        let mut mimetypes = HashMap::new();
        let mut extensions = HashMap::new();

        while let Some(Ok(line)) = lines.next() {
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

    pub fn extensions<T: Into<String>>(&self, mimetype: T) -> Option<&Vec<String>> {
        self.extensions.get(&mimetype.into())
    }

    pub fn mimetype<T: Into<String>>(&self, extension: T) -> Option<&String> {
        self.mimetypes.get(&extension.into())
    }
}
