use super::Error;
use serde::{ser, ser::Impossible, Serialize};

pub fn to_rlp<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: vec![Vec::new()],
    };
    value.serialize(&mut serializer)?;
    assert_eq!(serializer.output.len(), 1);
    Ok(serializer.output.pop().unwrap())
}

pub struct Serializer {
    output: Vec<Vec<u8>>,
}

impl Serializer {
    fn top(&mut self) -> &mut Vec<u8> {
        self.output.last_mut().unwrap()
    }

    fn push(&mut self) {
        self.output.push(Vec::new());
    }

    fn pop(&mut self) {
        let v = self.output.pop().unwrap();
        match v.len() {
            n if n <= 55 => {
                self.top().push(0xc0 + (n as u8));
                self.top().extend_from_slice(v.as_slice());
            }
            n => {
                let n = n as u64;
                let bytes = n.to_be_bytes();
                let zeros = n.leading_zeros() as usize / 8;
                let bytes = &bytes[zeros..];
                self.top().push(0xf7 + (bytes.len() as u8));
                self.top().extend_from_slice(bytes);
                self.top().extend_from_slice(v.as_slice());
            }
        }
    }
}

impl<'a> serde::Serializer for &'a mut Serializer {
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
        dbg!(v);
        dbg!(v.len());
        match v.len() {
            0 => self.top().push(0x80),
            1 if v[0] <= 0x7f => dbg!(self.top().push(v[0])),
            n if n <= 55 => {
                self.top().push(0x80 + (n as u8));
                self.top().extend_from_slice(v);
            }
            n => {
                let n = n as u64;
                let bytes = n.to_be_bytes();
                let zeros = n.leading_zeros() as usize / 8;
                let bytes = &bytes[zeros..];
                self.top().push(0xb7 + (bytes.len() as u8));
                self.top().extend_from_slice(bytes);
                self.top().extend_from_slice(v);
            }
        }
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
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
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

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
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
        self.pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
