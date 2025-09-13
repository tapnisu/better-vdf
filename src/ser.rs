use serde::Serialize;

use crate::error::{Error, Result};

pub struct Serializer {
    output: String,
    indexes: Vec<usize>,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        indexes: Vec::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> serde::ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.output += if v { "\"1\"" } else { "\"0\"" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{v}\"");
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{v}\"");
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{v}\"");
        Ok(())
    }

    fn serialize_char(self, v: char) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{v}\"");
        Ok(())
    }

    fn serialize_str(self, v: &str) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{v}\"");
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_none(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::prelude::v1::Result<Self::SerializeSeq, Self::Error> {
        self.output += "{";
        self.indexes.push(0);
        Ok(self)
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::prelude::v1::Result<Self::SerializeMap, Self::Error> {
        self.output += "{";
        self.indexes.push(0);
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());

        self.indexes
            .last()
            .copied()
            .unwrap()
            .serialize(&mut **self)?;

        self.output += "\t\t";

        value.serialize(&mut **self)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.indexes.pop();
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());
        self.output += "}";

        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());

        self.indexes
            .last()
            .copied()
            .unwrap()
            .serialize(&mut **self)?;

        self.output += "\t\t";

        value.serialize(&mut **self)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.indexes.pop();
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());
        self.output += "}";

        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());

        self.indexes
            .last()
            .copied()
            .unwrap()
            .serialize(&mut **self)?;

        self.output += "\t\t";

        value.serialize(&mut **self)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.indexes.pop();
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());
        self.output += "}";

        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedType)
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());

        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "\t\t";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.indexes.pop();
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());
        self.output += "}";
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());
        key.serialize(&mut **self)?;
        self.output += "\t\t";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.indexes.pop();
        self.output += "\n";
        self.output += &"\t".repeat(self.indexes.len());
        self.output += "}";
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedType)
    }
}
