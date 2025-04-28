/// Helper types to construct structured properties
use std::{convert::Infallible, fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

use crate::{Property, PropertyValue};

#[derive(Debug, Clone, Default)]
pub struct Address {
    pub post_office_box: String,
    pub extended_address: String,
    pub street_address: String,
    pub locality: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr)]
pub enum Telephone {
    Fax,
    Voice,
    Other(String),
}

impl From<Address> for PropertyValue {
    fn from(address: Address) -> Self {
        PropertyValue::Structured(
            [
                address.post_office_box,
                address.extended_address,
                address.street_address,
                address.locality,
                address.region,
                address.postal_code,
                address.country,
            ]
            .into_iter()
            .map(PropertyValue::String)
            .collect(),
        )
    }
}

impl From<[String; 7]> for Address {
    fn from(value: [String; 7]) -> Self {
        let [post_office_box, extended_address, street_address, locality, region, postal_code, country] =
            value;

        Self {
            post_office_box,
            extended_address,
            street_address,
            locality,
            region,
            postal_code,
            country,
        }
    }
}

impl TryFrom<Vec<String>> for Address {
    type Error = InvalidStructuredAddress;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let Ok(
            [post_office_box, extended_address, street_address, locality, region, postal_code, country],
        ) = TryInto::<[String; 7]>::try_into(value)
        else {
            return Err(InvalidStructuredAddress);
        };

        Ok(Self {
            post_office_box,
            extended_address,
            street_address,
            locality,
            region,
            postal_code,
            country,
        })
    }
}

#[derive(Error, Debug)]
#[error("Invalid strucutured address")]
pub struct InvalidStructuredAddress;

impl From<Address> for Property {
    fn from(address: Address) -> Self {
        Self::new("adr", None, "text", address)
    }
}

impl Display for Telephone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Telephone {
    fn as_ref(&self) -> &str {
        match self {
            Self::Fax => "fax",
            Self::Voice => "voice",
            Self::Other(other) => other.as_ref(),
        }
    }
}

impl FromStr for Telephone {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "fax" => Self::Fax,
            "voice" => Self::Voice,
            other => Self::Other(other.to_string()),
        })
    }
}
