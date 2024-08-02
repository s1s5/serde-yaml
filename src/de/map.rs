use serde::de::{DeserializeSeed, MapAccess};
use yaml_rust2::Yaml;

use super::{Deserializer, Error, Result};

pub struct YamlMap<'a> {
    pub iter: hashlink::linked_hash_map::Iter<'a, Yaml, Yaml>,
    pub next_value: Option<&'a Yaml>,
}

impl<'a, 'de> MapAccess<'de> for YamlMap<'a>
where
    'a: 'de,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.next_value = Some(value);
            seed.deserialize(&mut Deserializer { yaml: key }).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(&mut Deserializer { yaml: value })
        } else {
            Err(Error::ParseError)
        }
    }
}
