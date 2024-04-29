use std::fmt;
use std::fmt::Write;
use serde::de;
use uuid::Uuid;

pub fn deserialize_uuid_list<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
    where
        D: de::Deserializer<'de>,
{
    struct StringVecVisitor;

    impl<'de> de::Visitor<'de> for StringVecVisitor {
        type Value = Vec<Uuid>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a list of UUIDs")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(";") {
                let id = Uuid::parse_str(id).map_err(E::custom)?;
                ids.push(id);
            }
            Ok(ids)
        }
    }

    deserializer.deserialize_any(StringVecVisitor)
}