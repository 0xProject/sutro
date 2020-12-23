use crate::prelude::*;
use serde::{de, ser};
use smallvec::{smallvec, SmallVec};
use std::{fmt, iter::FromIterator, marker::PhantomData};

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(from = "Representation<T>", into = "Representation<T>")]
pub struct ValueOrArray<T: Clone>(SmallVec<[T; 1]>);

impl<T: Clone> From<T> for ValueOrArray<T> {
    fn from(value: T) -> Self {
        ValueOrArray(smallvec![value])
    }
}

impl<T: Clone> From<SmallVec<[T; 1]>> for ValueOrArray<T> {
    fn from(value: SmallVec<[T; 1]>) -> Self {
        ValueOrArray(value)
    }
}

impl<T: Clone> FromIterator<T> for ValueOrArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(SmallVec::from_iter(iter))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Representation<T: Clone> {
    None,
    One(T),
    Many(SmallVec<[T; 1]>),
}

impl<T: Clone> From<ValueOrArray<T>> for Representation<T> {
    fn from(mut value: ValueOrArray<T>) -> Self {
        match value.0.len() {
            0 => Representation::None,
            1 => Representation::One(value.0.pop().unwrap()),
            _ => Representation::Many(value.0),
        }
    }
}

impl<T: Clone> From<Representation<T>> for ValueOrArray<T> {
    fn from(value: Representation<T>) -> Self {
        match value {
            Representation::None => ValueOrArray(SmallVec::new()),
            Representation::One(value) => ValueOrArray(smallvec![value]),
            Representation::Many(vec) => ValueOrArray(vec),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::{assert_eq, *};
    use serde_json::{from_value, json, to_value};
    use std::iter::repeat;

    #[test]
    fn test_none() {
        let obj = repeat(42).take(0).collect::<ValueOrArray<u32>>();
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!(null));
        let de: ValueOrArray<u32> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_one() {
        let obj = repeat(42).take(1).collect::<ValueOrArray<u32>>();
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!(42));
        let de: ValueOrArray<u32> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_three() {
        let obj = repeat(42).take(3).collect::<ValueOrArray<u32>>();
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!([42, 42, 42]));
        let de: ValueOrArray<u32> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }
}
