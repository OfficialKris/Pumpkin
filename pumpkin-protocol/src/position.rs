use serde::{Deserialize, Serialize};

pub struct WorldPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Serialize for WorldPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let long = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);
        serializer.serialize_i64(long)
    }
}

impl<'de> Deserialize<'de> for WorldPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = WorldPosition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("An i64 int")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(WorldPosition {
                    x: (v >> 38) as i32,
                    y: (v << 52 >> 52) as i32,
                    z: (v << 26 >> 38) as i32,
                })
            }
        }
        deserializer.deserialize_i64(Visitor)
    }
}