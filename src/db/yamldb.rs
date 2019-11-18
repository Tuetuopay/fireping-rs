use serde_yaml;
use std::{fs, io, fmt};
use std::collections::HashMap;
use std::error::Error as StdError;

use crate::Target;
use crate::db::Db;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    YamlError(serde_yaml::Error),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Self::IoError(e) => e.description(),
            Self::YamlError(e) => e.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self { Self::IoError(e) }
}
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self { Self::YamlError(e) }
}

#[derive(Debug)]
pub struct YamlDb {
    targets: HashMap<String, String>,
}

impl YamlDb {
    pub fn new() -> Result<Self, Error> {
        let contents = fs::read_to_string("fireping.yml")?;
        let ret = serde_yaml::from_str(&contents)?;
        Ok(Self { targets: ret })
    }
}

impl Db<Error> for YamlDb {
    fn init(&self) -> Result<usize, Error> { Ok(0) }

    fn targets(&self) -> Result<Vec<Target>, Error> {
        Ok(self.targets.iter()
            .map(|(k, v)| Target { target: k.to_string(), name: v.to_string() })
            .collect())
    }
}
