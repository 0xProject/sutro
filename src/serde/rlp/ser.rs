/// Serde RLP serialization
///
/// There is no obvious single-pass way to serialize to RLP, especially
/// when data is deeply nested. Instead we use a stack of lists.
///
/// OPT: Consider alternative strategies, like
///   A) a single buffer with one or more fixup-passes through the output.
///   B) multiple passes through the data to compute the length first.
/// I suspect option B is faster and preferred since it reads less (only lengths
/// of fields are required) and writes much less (no moving data around). In
/// fact it could be done without allocations altogether.
use super::Error;
use crate::prelude::*;
use serde::{ser, ser::Impossible, Serialize};
use std::io::Write;

pub fn to_rlp<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(Vec::<u8>::new());
    value.serialize(&mut serializer)?;
    Ok(serializer.finish()?)
}

pub struct Serializer<W: Write> {
    output: W,
    stack:  Vec<Vec<u8>>,
}

impl<W: Write> Serializer<W> {
    pub fn new(output: W) -> Self {
        Self {
            output,
            stack: Vec::new(),
        }
    }

    pub fn finish(mut self) -> Result<W, Error> {
        require!(self.stack.is_empty(), Error::InvalidSerialization);
        self.output.flush()?;
        Ok(self.output)
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if let Some(vec) = self.stack.last_mut() {
            vec.extend_from_slice(bytes);
        } else {
            self.output.write_all(bytes)?;
        }
        Ok(())
    }

    fn push(&mut self) {
        self.stack.push(Vec::new());
    }

    fn pop(&mut self) -> Result<(), Error> {
        let v = self.stack.pop().unwrap();
        match v.len() as u64 {
            n if n <= 55 => self.write(&[0xc0 + (n as u8)])?,
            n => {
                let bytes = n.to_be_bytes();
                let zeros = n.leading_zeros() as usize / 8;
                let bytes = &bytes[zeros..];
                self.write(&[0xf7 + (bytes.len() as u8)])?;
                self.write(bytes)?;
            }
        }
        self.write(v.as_slice())?;
        Ok(())
    }
}

impl<'a, W: Write> serde::Serializer for &'a mut Serializer<W> {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    fn is_human_readable(&self) -> bool {
        false
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(if v { 0 } else { 1 })
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v == 0 {
            self.serialize_bytes(&[0])
        } else {
            let bytes = v.to_be_bytes();
            let leading_zeros = (v.leading_zeros() / 8) as usize;
            let meat = &bytes[leading_zeros..];
            self.serialize_bytes(meat)
        }
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buffer = [0; 4];
        let str = v.encode_utf8(&mut buffer);
        self.serialize_str(str)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match v.len() as u64 {
            0 => self.write(&[0x80])?,
            1 if v[0] <= 0x7f => {}
            n if n <= 55 => self.write(&[0x80 + (n as u8)])?,
            n => {
                let bytes = n.to_be_bytes();
                let zeros = n.leading_zeros() as usize / 8;
                let bytes = &bytes[zeros..];
                self.write(&[0xb7 + (bytes.len() as u8)])?;
                self.write(bytes)?;
            }
        }
        self.write(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
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
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.push();
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.push();
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.push();
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!();
        // Err(Error::UnsupportedType)
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.pop()?;
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.pop()?;
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.pop()?;
        Ok(())
    }
}
