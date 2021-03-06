use crate::utils;
use std::path::{Path, PathBuf};

pub struct Location {
    pub path: PathBuf,
}

impl Location {
    pub fn new(path: &str) -> Self {
        path = utils::no_leading_slash(path);

        Self {
            path: Path::new("./out").join(path),
        }
    }

    pub fn concat(&self, path: &str) -> Self {
        path = utils::no_leading_slash(path);

        Self {
            path: self.path.join(path),
        }
    }
}
