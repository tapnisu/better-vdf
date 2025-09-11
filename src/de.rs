use std::str::FromStr;

use serde::{
    de::{MapAccess, SeqAccess},
    Deserialize,
};

use crate::error::{Error, Result};

pub struct Deserializer<'de> {
    original: &'de str,
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer {
            original: input,
            input,
        }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer
        .input
        .chars()
        .filter(|&x| x != '\t' && x != '\n' && x != '\r' && x != ' ')
        .count()
        == 0
    {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de> Deserializer<'de> {
    fn peek_char(&self) -> Result<char> {
        self.input
            .chars()
            .next()
            .ok_or(Error::Eof)
            .or_else(|x| panic!("{x:?}"))
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn peek_real_char(&self) -> Result<char> {
        self.input
            .chars()
            .find(|&x| x != '\t' && x != '\n' && x != '\r' && x != ' ')
            .ok_or(Error::Eof)
        // .or_else(|x| panic!("{x:?}"))
    }

    fn next_real_char(&mut self) -> Result<char> {
        let mut ch = self.next_char()?;

        while ch == '\t' || ch == '\n' || ch == '\r' || ch == ' ' {
            ch = self.next_char()?;
        }

        Ok(ch)
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if self.next_real_char()? != '"' {
            return Err(Error::ExpectedString);
        }
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => {
                // Err(Error::Eof)
                panic!("{:?}", Error::Eof)
            }
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        let str = self.parse_string()?;

        match str {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(Error::ExpectedBoolean),
        }
    }

    fn parse_value<T>(&mut self) -> Result<T>
    where
        T: FromStr,
    {
        let str = self.parse_string()?;

        str.parse::<T>().map_err(|_| Error::ExpectedInteger)
    }
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NonSelfDescribing)
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_value()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_value()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_value()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_value()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_value()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_value()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_value()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_value()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_value()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_value()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_str<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_option<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.next_real_char()? == '{' {
            let value = visitor.visit_seq(VdfSeq::new(self))?;

            if self.next_real_char()? == '}' {
                Ok(value)
            } else {
                Err(Error::ExpectedArrayEnd)
            }
        } else {
            Err(Error::ExpectedArray)
        }
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let begin = self.original.as_ptr() == self.input.as_ptr();

        if begin || self.next_real_char()? == '{' {
            let value = visitor.visit_map(VdfMap::new(self))?;

            if begin || self.next_real_char()? == '}' {
                Ok(value)
            } else {
                Err(Error::ExpectedMapEnd)
            }
        } else {
            Err(Error::ExpectedMap)
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct VdfSeq<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    index: usize,
}

impl<'a, 'de> VdfSeq<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        VdfSeq { de, index: 0 }
    }
}

impl<'de, 'a> SeqAccess<'de> for VdfSeq<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::prelude::v1::Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.de.peek_real_char()? {
            '}' => Ok(None),
            '"' => {
                //
                let ind: usize = self.de.parse_value()?;

                if ind != self.index {
                    println!("Expected index {}, found {ind}", self.index);
                    return Err(Error::ArrayIndex);
                }

                self.index += 1;

                seed.deserialize(&mut *self.de).map(Some)
            }
            _ => Err(Error::SeqSyntax),
        }
    }
}

struct VdfMap<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    root: bool,
    initialized: bool,
}

impl<'a, 'de> VdfMap<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        VdfMap {
            root: de.input.as_ptr() == de.original.as_ptr(),
            de,
            initialized: false,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for VdfMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> std::prelude::v1::Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let res = self.de.peek_real_char();

        if self.root && self.initialized && res.is_err() || res? == '}' {
            return Ok(None);
        }

        self.initialized = true;

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
