use serde::de::{DeserializeSeed, SeqAccess};
use yaml_rust2::Yaml;

use super::{Deserializer, Error, Result};

pub struct YamlVec<'a> {
    pub iter: std::slice::Iter<'a, Yaml>,
}

impl<'de, 'a> SeqAccess<'de> for YamlVec<'a>
where
    'a: 'de,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(e) = self.iter.next() {
            seed.deserialize(&mut Deserializer { yaml: e }).map(Some)
        } else {
            Ok(None)
        }
    }
}
