use serde::de::{self, IntoDeserializer, Visitor};
use yaml_rust2::Yaml;

use super::map::YamlMap;
use super::r#enum::YamlEnum;
use super::vec::YamlVec;
use super::{Error, Result};

pub struct Deserializer<'a> {
    pub yaml: &'a Yaml,
}

impl<'a> Deserializer<'a> {
    fn unexpected(&self) -> de::Unexpected {
        match self.yaml {
            Yaml::Real(_) => {
                if let Some(f) = self.yaml.as_f64() {
                    de::Unexpected::Float(f)
                } else {
                    de::Unexpected::Other("bad float")
                }
            }
            Yaml::Integer(i) => de::Unexpected::Signed(*i),
            Yaml::String(s) => de::Unexpected::Str(s),
            Yaml::Boolean(b) => de::Unexpected::Bool(*b),
            Yaml::Array(_a) => de::Unexpected::Seq,
            Yaml::Hash(_m) => de::Unexpected::Map,
            Yaml::Alias(_a) => de::Unexpected::Other("alias"),
            Yaml::Null => de::Unexpected::Unit,
            Yaml::BadValue => de::Unexpected::Other("bad value"),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.yaml {
            Yaml::Real(_) => {
                if let Some(f) = self.yaml.as_f64() {
                    visitor.visit_f64(f)
                } else {
                    Err(de::Error::custom(format!("bad float. {:?}", self.yaml)))
                }
            }
            Yaml::Integer(i) => visitor.visit_i64(*i),
            Yaml::String(s) => visitor.visit_borrowed_str(s),
            Yaml::Boolean(b) => visitor.visit_bool(*b),
            Yaml::Array(a) => visitor.visit_seq(YamlVec { iter: a.iter() }),
            Yaml::Hash(m) => visitor.visit_map(YamlMap {
                iter: m.iter(),
                next_value: None,
            }),
            Yaml::Alias(_a) => Err(de::Error::custom(format!(
                "Unsupported alias. {:?}",
                self.yaml
            ))),
            Yaml::Null => visitor.visit_unit(),
            Yaml::BadValue => Err(de::Error::custom(format!("bad value. {:?}", self.yaml))),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_bool() {
            visitor.visit_bool(v)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"bool"))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_i8(v as i8)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"i8"))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_i16(v as i16)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"i16"))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_i32(v as i32)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"i32"))
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_i64(v)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"i64"))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_u8(v as u8)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"u8"))
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_u16(v as u16)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"u16"))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_u32(v as u32)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"u32"))
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_i64() {
            visitor.visit_u64(v as u64)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"u64"))
        }
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_f64() {
            visitor.visit_f32(v as f32)
        } else if let Some(v) = self.yaml.as_i64() {
            visitor.visit_f32(v as f32)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"f32"))
        }
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.yaml.as_f64() {
            visitor.visit_f64(v)
        } else if let Some(v) = self.yaml.as_i64() {
            visitor.visit_f64(v as f64)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"f64"))
        }
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.yaml.as_str() {
            if value.len() == 1 {
                visitor.visit_char(value.chars().next().unwrap())
            } else {
                Err(de::Error::invalid_value(
                    self.unexpected(),
                    &"string length must be 1",
                ))
            }
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"string"))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.yaml.as_str() {
            visitor.visit_borrowed_str(value)
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"string"))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.yaml.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.yaml.is_null() {
            visitor.visit_none()
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"null"))
        }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(vec) = self.yaml.as_vec() {
            visitor.visit_seq(YamlVec { iter: vec.iter() })
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"seq"))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(m) = self.yaml.as_hash() {
            visitor.visit_map(YamlMap {
                iter: m.iter(),
                next_value: None,
            })
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"map"))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.yaml.as_str() {
            visitor.visit_enum(value.into_deserializer())
        } else if let Some(m) = self.yaml.as_hash() {
            let mut it = m.iter();
            if let Some((key, value)) = it.next() {
                if it.next().is_some() {
                    Err(Error::ParseError)
                } else {
                    visitor.visit_enum(YamlEnum { key, value })
                }
            } else {
                Err(Error::ParseError)
            }
        } else {
            Err(de::Error::invalid_type(self.unexpected(), &"enum"))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i128<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("i128 is not supported"))
    }

    fn deserialize_u128<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("u128 is not supported"))
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}
