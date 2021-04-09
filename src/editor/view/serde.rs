pub mod panel_kind {
    use super::super::*;
    use ::serde::de::{self, Deserializer, Visitor};
    use ::serde::ser::Serializer;
    use std::convert::TryFrom as _;
    use std::fmt;

    pub fn serialize<S>(kind: &PanelKind, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8((*kind).into())
    }

    pub fn deserialize<'a, D>(deserializer: D) -> Result<PanelKind, D::Error>
    where
        D: Deserializer<'a>,
    {
        struct PanelKindVisitor;

        impl<'de> Visitor<'de> for PanelKindVisitor {
            type Value = PanelKind;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid PanelKind value")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                PanelKind::try_from(value as u8)
                    .map_err(|_| E::invalid_value(de::Unexpected::Unsigned(value), &self))
            }
        }

        deserializer.deserialize_u8(PanelKindVisitor)
    }
}

pub mod field {
    use super::super::*;
    use ::serde::de::{self, Deserializer, Visitor};
    use ::serde::ser::Serializer;
    use citrus_common::format::fldx;
    use std::fmt;

    pub fn serialize<S>(field: &Rc<Field>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = Vec::new();
        fldx::encode(field, &mut buf).unwrap();

        serializer.serialize_bytes(&buf)
    }

    pub fn deserialize<'a, D>(deserializer: D) -> Result<Rc<Field>, D::Error>
    where
        D: Deserializer<'a>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Rc<Field>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid Field in fldx format")
            }

            fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut cursor = std::io::Cursor::new(bytes);

                fldx::decode(&mut cursor)
                    .map_err(|_| E::invalid_value(de::Unexpected::Bytes(bytes), &self))
                    .map(Rc::new)
            }
        }

        deserializer.deserialize_bytes(FieldVisitor)
    }
}
