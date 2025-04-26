use serde::{
    ser::{Error, SerializeSeq as _},
    Serialize,
};

use crate::{Property, Vcard};

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
        seq.serialize_element(&self.parameters)?;
        seq.serialize_element(&self.value_type)?;
        self.values
            .iter()
            .try_for_each(|v| seq.serialize_element(v))?;

        seq.end()
    }
}
