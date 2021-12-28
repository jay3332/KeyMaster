#[macro_export]
macro_rules! serde_bitflags {
    ($tt:ident) => {
        impl Serialize for $tt {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u64(self.bits)
            }
        }

        use crate::macros::serde_bitflags::U64Visitor;
        use serde::Deserializer;

        impl<'de> Deserialize<'de> for $tt {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Self {
                    bits: deserializer.deserialize_u64(U64Visitor)?,
                })
            }
        }
    };
}

use serde::de::Visitor;
use std::fmt::Formatter;

pub struct U64Visitor;

impl Visitor<'_> for U64Visitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("64-bit unsigned integer")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}
