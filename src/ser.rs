use serde::{
    ser::{Error, SerializeMap, SerializeSeq as _},
    Serialize,
};

use crate::{Parameters, Property, PropertyValue, Vcard};

impl Serialize for Vcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut vcard = serializer.serialize_seq(Some(2))?;
        vcard.serialize_element("vcard")?;

        struct VersionPrefix<'a, T>(&'a str, &'a [T]);
        impl<T> Serialize for VersionPrefix<'_, T>
        where
            T: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(self.1.len() + 1))?;

                seq.serialize_element(&Property::new("version", None, "text", self.0))?;

                self.1
                    .iter()
                    .try_for_each(|prop| seq.serialize_element(prop))?;

                seq.end()
            }
        }

        vcard.serialize_element(&VersionPrefix(&self.version, &self.properties))?;
        vcard.end()
    }
}

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.values.is_empty() {
            return Err(S::Error::custom(
                "at least one value must be present in a property",
            ));
        }

        let mut seq = serializer.serialize_seq(Some(3 + self.values.len()))?;

        seq.serialize_element(&self.name)?;
        seq.serialize_element(&MapToOneOrMany(&self.parameters))?;
        seq.serialize_element(&self.value_type)?;
        self.values
            .iter()
            .try_for_each(|v| seq.serialize_element(v))?;

        seq.end()
    }
}

struct MapToOneOrMany<'a>(&'a Parameters);

impl Serialize for MapToOneOrMany<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (key, value) in self.0 {
            match value.as_slice() {
                [] => {
                    return Err(S::Error::custom(
                        "vcard property parameter is an empty array",
                    ))
                }
                [single] => map.serialize_entry(key, single)?,
                multiple => map.serialize_entry(key, multiple)?,
            }
        }

        map.end()
    }
}

impl Serialize for PropertyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PropertyValue::String(string) => serializer.serialize_str(string),
            PropertyValue::Bool(boolean) => serializer.serialize_bool(*boolean),
            PropertyValue::Integer(int) => serializer.serialize_i64(*int),
            PropertyValue::Float(float) => serializer.serialize_f64(*float),
            PropertyValue::Structured(property_values) => match property_values.as_slice() {
                [] => Err(S::Error::custom("empty structured value")),
                [single] => single.serialize(serializer),
                many => {
                    let mut seq = serializer.serialize_seq(Some(many.len()))?;
                    for value in many {
                        seq.serialize_element(value)?
                    }
                    seq.end()
                }
            },
        }
    }
}
