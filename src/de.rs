use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize,
};

use crate::{Property, PropertyValue, Vcard};

impl<'de> Deserialize<'de> for Vcard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VcardVisitor;
        impl<'de> Visitor<'de> for VcardVisitor {
            type Value = Vcard;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an RFC 7095 jCard")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let header: Option<String> = seq.next_element()?;
                match header.as_deref() {
                    Some("vcard") => {}
                    Some(other) => {
                        return Err(A::Error::invalid_value(
                            Unexpected::Str(other),
                            &r#"a "vcard" header string"#,
                        ))
                    }
                    None => {
                        return Err(A::Error::invalid_value(
                            Unexpected::Seq,
                            &r#"a non-empty array starting with a "vcard" header"#,
                        ))
                    }
                }

                let mut version = String::default();
                let Some(mut properties) = seq.next_element::<Vec<Property>>()? else {
                    return Err(A::Error::invalid_value(
                        Unexpected::Seq,
                        &r#"an array of jCard properties as the second element"#,
                    ));
                };

                let mut version_index = None;

                for (i, property) in properties.iter_mut().enumerate() {
                    if property.name.to_lowercase().as_str() != "version" {
                        continue;
                    }

                    version_index = Some(i);

                    version = match property.values.as_slice() {
                        [PropertyValue::String(_)] => {
                            let PropertyValue::String(moved_string) = property.values.remove(0)
                            else {
                                unreachable!()
                            };
                            moved_string
                        }
                        [PropertyValue::Structured(structured)] => match structured.as_slice() {
                            [_] => {
                                let PropertyValue::String(moved_string) = property.values.remove(0)
                                else {
                                    unreachable!()
                                };
                                moved_string
                            }

                            [] | [_, _, ..] => {
                                return Err(A::Error::invalid_length(
                                    property.values.len(),
                                    &"a non-structured version property",
                                ))
                            }
                        },
                        [] | [_, _, ..] => {
                            return Err(A::Error::invalid_length(
                                property.values.len(),
                                &"exactly one value in the jCard version property",
                            ))
                        }
                    };

                    break;
                }

                version_index.map(|i| properties.remove(i));

                Ok(Vcard {
                    version,
                    properties,
                })
            }
        }

        deserializer.deserialize_seq(VcardVisitor)
    }
}

impl<'de> Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PropertyVisitor;

        impl<'de> Visitor<'de> for PropertyVisitor {
            type Value = Property;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an RFC 7095 jCard property")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                /// The number of elements before the `values` array starts at this level in the property
                const FIXED_ELEMENTS: usize = 3;

                let mut counter = (0..).into_iter();
                let mut len_err = || {
                    Err(A::Error::invalid_length(
                        counter.next().unwrap(),
                        &"an array of at least 4 elements",
                    ))
                };

                let Some(name) = seq.next_element()? else {
                    return len_err();
                };
                let Some(parameters) = seq.next_element()? else {
                    return len_err();
                };
                let Some(value_type) = seq.next_element()? else {
                    return len_err();
                };

                let mut values = seq
                    .size_hint()
                    .map(|len| Vec::with_capacity(len.saturating_sub(FIXED_ELEMENTS)))
                    .unwrap_or_default();

                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }

                if values.is_empty() {
                    return Err(A::Error::invalid_length(
                        3,
                        &"at least one value of the jCard property",
                    ));
                }

                Ok(Property {
                    name,
                    parameters,
                    value_type,
                    values,
                })
            }
        }

        deserializer.deserialize_seq(PropertyVisitor)
    }
}
