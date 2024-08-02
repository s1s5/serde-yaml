pub mod deserializer;
mod r#enum;
pub mod error;
mod map;
mod vec;

pub use deserializer::Deserializer;
pub use error::{Error, Result};

use yaml_rust2::{Yaml, YamlLoader};

pub fn parse_yaml(s: &str) -> Result<Vec<Yaml>> {
    YamlLoader::load_from_str(s).map_err(Error::YamlError)
}

pub fn from_yaml<'a, T>(yaml: &'a Yaml) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    T::deserialize(&mut Deserializer { yaml })
}
