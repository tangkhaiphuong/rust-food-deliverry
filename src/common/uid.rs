use std::error::Error;
use std::str;

use base58::*;

/*
 * @author           Viet Tran <viettranx@gmail.com>
 * @copyright        2019 Viet Tran <viettranx@gmail.com>
 * @license          Apache-2.0
 */

// UID is method to generate a virtual unique identifier for whole system
// its structure contains 62 bits:  LocalID - ObjectType - ShardID
// 32 bits for Local ID, max (2^32) - 1
// 10 bits for Object Type
// 18 bits for Shard ID

#[derive(Clone, Copy, Default, Debug)]
pub struct Uid {
    local_id: u32,
    object_type: u32,
    shard_id: u32,
}

impl PartialEq<Self> for Uid {
    fn eq(&self, other: &Self) -> bool {
        if self.local_id != other.local_id {
            return false;
        }
        if self.object_type != other.object_type {
            return false;
        }
        if self.shard_id != other.shard_id {
            return false;
        }
        return true;
    }
}

impl Eq for Uid {}

impl Uid {
    pub fn new(local_id: u32, object_type: u32, shard_id: u32) -> Self {
        return Self {
            local_id,
            object_type,
            shard_id,
        };
    }

    pub fn local_id(&self) -> u32 {
        return self.local_id;
    }

    pub fn object_type(&self) -> u32 {
        return self.object_type;
    }

    pub fn shard_id(&self) -> u32 {
        return self.shard_id;
    }

    pub fn decomose<T: Into<String>>(s: T) -> Result<Uid, Box<dyn Error + Send + Sync>> {
        let uid: u32 = s.into().parse()?;

        if (1 << 18) > uid {
            return Err("wrong uid".to_string().into());
        }

        // x = 1110 1110 0101 => x >> 4 = 1110 1110 & 0000 1111 = 1110
        let u = Uid {
            local_id: (uid >> 28) as u32,
            object_type: (uid >> 18 & 0x3FF) as u32,
            shard_id: (uid >> 0 & 0x3FFFF) as u32,
        };

        return Ok(u);
    }

    pub fn from_base58<T: Into<String>>(s: T) -> Result<Uid, Box<dyn Error + Send + Sync>> {
        return match s.into().from_base58() {
            Ok(ref val) => Self::decomose(str::from_utf8(val.as_slice())?),
            Err(e) => Err(format!("{:?}", e).into()),
        };
    }
}

impl ToString for Uid {
    fn to_string(&self) -> String {
        let val = (self.local_id as u32) << 28 | (self.object_type) << 18 | (self.shard_id) << 0;
        let x = val.to_string().into_bytes();
        let result = x.to_base58();
        return result;
    }
}

impl Into<String> for Uid {
    fn into(self) -> String {
        self.to_string()
    }
}

pub mod uid {
    use core::fmt;

    use serde::{de, ser};

    use crate::common::Uid;

    pub fn serialize<S>(dt: &Uid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = dt.to_string();
        serializer.serialize_str(&value)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Uid, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_string(UidVisitor)?)
    }

    #[doc(hidden)]
    #[derive(Debug)]
    pub struct UidVisitor;

    impl<'de> de::Visitor<'de> for UidVisitor {
        type Value = Uid;

        // use chrono::serde::ts_seconds;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unix timestamp in seconds")
        }

        fn visit_str<E>(self, value: &str) -> Result<Uid, E>
        where
            E: de::Error,
        {
            return match Uid::from_base58(value) {
                Ok(value) => Ok(value),
                Err(err) => Err(E::custom(err)),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::uid::Uid;

    #[test]
    fn test_to_string() {
        assert_eq!(Uid::new(1, 1, 1).to_string(), "e532qos8jjM2");
        assert_eq!(Uid::new(3, 1, 5).to_string(), "iUwqQA5kiz9C");
    }

    #[test]
    fn test_from_base58() {
        assert_eq!(Uid::from_base58("e532qos8jjM2").unwrap(), Uid::new(1, 1, 1));
        assert_eq!(Uid::from_base58("iUwqQA5kiz9C").unwrap(), Uid::new(3, 1, 5));
    }

    #[test]
    fn test_from_base58_and_to_string_range() {
        for i in 1..256 {
            assert_eq!(
                Uid::from_base58(Uid::new(i, 1, 1)).unwrap(),
                Uid::new(i, 1, 1)
            );
        }
    }
}
