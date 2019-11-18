pub mod sqlitedb;
pub mod yamldb;

use crate::Target;
use std::error::Error;

pub trait Db<T: Error> {
    fn init(&self) -> Result<usize, T>;
    fn targets(&self) -> Result<Vec<Target>, T>;
}
