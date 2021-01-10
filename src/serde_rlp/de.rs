use super::Error;
use serde::{de, Deserialize};

pub fn from_rlp<'de, T>(rlp: &'de [u8]) -> Result<T, Error>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_rlp(rlp);
    let t = T::deserialize(&mut deserializer)?;
    deserializer.finish()?;
    Ok(t)
}

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_rlp(input: &'de [u8]) -> Self {
        Self { input }
    }

    pub fn finish(&mut self) -> Result<(), Error> {
        if self.input.is_empty() {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }

    fn read(&mut self, len: usize) -> Result<&'de [u8], Error> {
        if self.input.len() >= len {
            let (result, remaining) = self.input.split_at(len);
            self.input = remaining;
            Ok(result)
        } else {
            Err(Error::UnexpectedEnd)
        }
    }

    pub fn parse_bytes(&mut self) -> Result<&'de [u8], Error> {
        let next = self.read(1)?;
        match next[0] {
            b if b <= 0x7f => Ok(next),
            b if b <= 0xb7 => self.read((b - 0x80) as usize),
            b if b <= 0xbf => {
                let prefix = self.read((b - 0xb7) as usize)?;
                let length = {
                    let mut buffer = [0; 8];
                    buffer[(8 - prefix.len())..].copy_from_slice(prefix);
                    u64::from_be_bytes(buffer) as usize
                };
                self.read(length)
            }
            _ => Err(Error::UnexpectedList),
        }
    }

    pub fn parse_list(&mut self) -> Result<&'de [u8], Error> {
        match self.read(1)?[0] {
            b if b <= 0xbf => Err(Error::UnexpectedBytes),
            b if b <= 0xf7 => self.read((b - 0xc0) as usize),
            b => {
                let prefix = self.read((b - 0xf7) as usize)?;
                dbg!(&prefix);
                let length = {
                    let mut buffer = [0; 8];
                    buffer[(8 - prefix.len())..].copy_from_slice(prefix);
                    u64::from_be_bytes(buffer) as usize
                };
                dbg!(&length);
                self.read(length)
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        false
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        dbg!(hex::encode(self.input));
        let bytes = self.parse_bytes()?;
        dbg!(hex::encode(bytes));
        let str = std::str::from_utf8(bytes)?;
        dbg!(str);
        visitor.visit_str(str)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        dbg!(hex::encode(self.input));
        let bytes = self.parse_bytes()?;
        dbg!(hex::encode(bytes));
        visitor.visit_bytes(bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        dbg!(len);
        struct Access<'de> {
            deserializer: Deserializer<'de>,
            len:          usize,
        }

        impl<'de> de::SeqAccess<'de> for Access<'de> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = de::DeserializeSeed::deserialize(seed, &mut self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        let slice = self.parse_list()?;
        visitor.visit_seq(Access {
            deserializer: Deserializer { input: slice },
            len,
        })
        // TODO: Check for trailing data
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        dbg!(name, fields);
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }
}
