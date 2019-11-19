use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub enum RemovableValue<T: Clone> {
    Default,
    Remove,
    Specified(T),
}

impl<T: Clone> RemovableValue<T> {
    pub(crate) fn is_default(&self) -> bool {
        match self {
            RemovableValue::Default => true,
            _ => false,
        }
    }
}

impl<T: Clone> From<T> for RemovableValue<T> {
    fn from(value: T) -> Self {
        RemovableValue::Specified(value)
    }
}

impl From<&str> for RemovableValue<String> {
    fn from(value: &str) -> Self {
        RemovableValue::Specified(value.to_string())
    }
}

// impl<T> From<T> for RemovableValue<T>
// where
//     T: Into<crate::UrlData> + Clone,
// {
//     fn from(value: T) -> Self {
//         RemovableValue::Specified(value)
//     }
// }

impl<T: Clone> Default for RemovableValue<T> {
    fn default() -> Self {
        RemovableValue::Default
    }
}

impl<T> Serialize for RemovableValue<T>
where
    T: Serialize + Clone,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RemovableValue::Specified(ref value) => serializer.serialize_some(value),
            RemovableValue::Default => serializer.serialize_none(),
            RemovableValue::Remove => serializer.serialize_none(),
        }
    }
}

struct RemovableValueVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for RemovableValueVisitor<T>
where
    T: Deserialize<'de> + Clone,
{
    type Value = RemovableValue<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("option")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RemovableValue::Remove)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RemovableValue::Remove)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|v| RemovableValue::Specified(v))
    }

    #[doc(hidden)]
    fn __private_visit_untagged_option<D>(self, deserializer: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Ok(match T::deserialize(deserializer) {
            Ok(v) => RemovableValue::Specified(v),
            _ => RemovableValue::Remove,
        })
    }
}

impl<'de, T> Deserialize<'de> for RemovableValue<T>
where
    T: Deserialize<'de> + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(RemovableValueVisitor {
            marker: PhantomData,
        })
    }
}
