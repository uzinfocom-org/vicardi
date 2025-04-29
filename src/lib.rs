//!<header>
//!<img src="https://raw.githubusercontent.com/uzinfocom-org/website/main/src/images/logo.svg" alt="logo" height="100" align="left" style="padding-right: 1em;">
//!<h1 style="display: inline">Vicardi</h1>
//!
//!jCard (vCard in JSON format) serde serialization and deserialization.
//!
//![![GitHub top language](https://img.shields.io/github/languages/top/uzinfocom-org/vicardi?style=flat-square&logo=github)](https://github.com/uzinfocom-org/vicardi)
//![![Chat](https://img.shields.io/badge/Chat-grey?style=flat-square&logo=telegram)](https://t.me/xinuxuz)
//![![Test CI](https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml/badge.svg)](https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml)
//!
//!</header>
//!
//! # The jCard format
//!
//! The jCard format is quite simple. A jCard is an arrary with 2 elements:
//!
//! - The string "vcard"
//! - A nested array with the vCard properties
//!   - Each element in the array is another array with at least 4 elements:
//!     - Name (e.g. "fn")
//!     - Properties (a string to string map)
//!     - Value type (e.g. "text")
//!     - 1+ values of that property
//!
//! ```json
//! [
//!   "vcard",
//!   [
//!     ["version", {}, "text", "4.0"],
//!     ["fn", {}, "text", "Vicardi"],
//!     ["categories", {}, "text", "rust", "serde"],
//!     ...
//!   ]
//! ]
//! ```
//!
//! The entire array is represented by the [`Vcard`] type. Each property is a [`Property`] and it can have 1 or more
//! [`PropertyValue`]s.
//!
//! **A note on the version property:**
//!
//! The RFC requires that the first element in the array is a version property. At the moment, this crate does not
//! enforce any rules regarding the position or number of version properties. However, the first occurance of the
//! version property is removed from the array during deserialization. The value of the version is stored in the
//! [`Vcard::version`] field.
//!
//! During serialization, the value of [`Vcard::version`] is placed at index 0 in the properties array.
use serde::Deserialize;
use serde_with::serde_as;
use std::collections::HashMap;

mod de;
mod ser;

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub use structured::*;
pub mod structured;

pub type Parameters = HashMap<String, Vec<String>>;

/// A jCard serde type
#[derive(Debug, Clone, PartialEq)]
pub struct Vcard {
    /// A parsed out jCard version property.
    pub version: String,
    /// jCard properties
    ///
    /// # Notes
    ///
    /// - Do not include the `version` type property in this array. Instead, set the [`Vcard::version`] property.
    ///   `["version,{},"text",version]` will be inserted by the [`Serialize`] implementation.
    pub properties: Vec<Property>,
}

impl Default for Vcard {
    fn default() -> Self {
        Self {
            version: "4.0".into(),
            properties: Vec::default(),
        }
    }
}

/// An entry in the jCard.
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    /// Aka the type. E.g. `"fn"`.
    pub name: String,

    /// The list of parameters such as the laguage or the preference value.
    ///
    /// The hashmap's value [`Vec`] will be converted to a single string in JSON if it only has 1 element.
    ///
    /// ```rust
    /// # use vicardi::*;
    /// # use serde_json::json;
    /// # fn main() -> anyhow::Result<()> {
    /// let json_array = json!(["", {"foo": ["structured"]}, "", ""]);
    /// let property: Property = serde_json::from_value(json_array)?;
    /// let structured = &property.parameters;
    /// assert_eq!(
    ///     structured,
    ///     &parameters! {"foo" => "structured"}
    /// );
    ///
    /// let json_string = json!(["", {"foo": "structured"}, "", ""]);
    /// let string = serde_json::to_value(property)?;
    /// assert_eq!(
    ///     string,
    ///     json_string
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub parameters: Parameters,

    /// The value type. E.g. `"text"`
    pub value_type: String,

    /// Either a single or multiple values of the jCard property.
    ///
    /// When the array has multiple elements, they are appended at the level of the property array in jCard format:
    ///
    /// ```json
    /// ["categories", {}, "text",
    ///   "rust", "serde"
    /// ]
    /// ```
    ///
    /// Where rust and serde are [`PropertyValue::String`]
    ///
    /// For a structured property, use [`PropertyValue::Structured`]. See [`Property::new_adr`] for an example of a
    /// structured property.
    pub values: Vec<PropertyValue>,
}

/// A [`Property::values`] can either be a single value or an array of values. See an example json representation for
/// each variant.
///
/// Per [RFC 7095, Section 3.3.1.3](https://datatracker.ietf.org/doc/html/rfc7095#section-3.3.1.3)
/// > The array element values MUST have the primitive type that matches the jCard type identifier. In RFC6350,
/// > there are only structured text values and thus only JSON strings are used. For example, extensions may define
/// > structured number or boolean values, where JSON number or boolean types MUST be used.
/// >
/// > ...
/// >
/// > Although it is allowed for a structured property value to hold just one component, it is RECOMMENDED to represent
/// > it as a single text value instead, omitting the array completely.  Nevertheless, a simple implementation MAY
/// > choose to retain the array, with a single text value as its element.
///
/// ```rust
/// # use vicardi::*;
/// # use serde_json::json;
/// # fn main() -> anyhow::Result<()> {
/// let json_array = json!(["structured"]);
/// let structured: PropertyValue = serde_json::from_value(json_array)?;
/// assert_eq!(
///     structured,
///     PropertyValue::Structured(vec!["structured".into()])
/// );
///
/// let json_string = json!("structured");
/// let string = serde_json::to_value(structured)?;
/// assert_eq!(
///     string,
///     json_string
/// );
/// # Ok(())
/// # }
/// ```
#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    /// ```json
    /// ["fn", {}, "text", "Vicardi"]
    /// ```
    String(String),

    /// ```json
    /// ["x-use-rust", {}, "boolean", true]
    /// ```
    Bool(bool),

    /// ```json
    /// ["x-karma-points", {}, "integer", 42]
    /// ```
    Integer(i64),

    /// ```json
    /// ["x-grade", {}, "integer", 1.3]
    /// ```
    Float(f64),

    /// Example structured string property value:
    /// ```json
    /// ["org", {}, "text",
    ///     ["Organization", "Department", "etc"]
    /// ]
    /// ```
    Structured(Vec<PropertyValue>),
}

impl Vcard {
    /// Appends a property to the vCard.
    ///
    /// Check the examples for [`Property`]'s constructors for examples of how to easily append different vCard
    /// properties.
    pub fn push(&mut self, property: impl Into<Property>) {
        self.properties.push(property.into());
    }
}

impl Property {
    /// Creates a new property, where [`Property::values`] is a `vec![value]`.
    pub fn new(
        name: impl ToString,
        parameters: impl Into<Option<Parameters>>,
        value_type: impl ToString,
        value: impl Into<PropertyValue>,
    ) -> Self {
        Self::new_multivalued(name, parameters, value_type, vec![value.into()])
    }

    pub fn new_multivalued(
        name: impl ToString,
        parameters: impl Into<Option<Parameters>>,
        value_type: impl ToString,
        values: Vec<PropertyValue>,
    ) -> Self {
        Self {
            name: name.to_string(),
            parameters: parameters.into().unwrap_or_default(),
            value_type: value_type.to_string(),
            values,
        }
    }

    /// # Example
    ///
    /// ```rust
    /// # use vicardi::*;
    /// # use serde_json::json;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut vcard = Vcard::default();
    /// vcard.push(Property::new_fn("John Doe", None));
    ///
    /// let json = json!([
    ///     "vcard",
    ///     [
    ///         ["version", {}, "text", "4.0"],
    ///         ["fn", {}, "text", "John Doe"]
    ///     ]
    /// ]);
    ///
    /// let parsed: Vcard = serde_json::from_value(json.clone())?;
    ///
    /// assert_eq!(serde_json::to_value(&vcard)?, json);
    /// assert_eq!(parsed, vcard);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_fn(formatted: impl ToString, parameters: impl Into<Option<Parameters>>) -> Self {
        Self::new("fn", parameters, "text", formatted)
    }

    /// # Example
    /// ```rust
    /// # use vicardi::*;
    /// # use serde_json::json;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut vcard = Vcard::default();
    ///
    /// let address = Address {
    ///     street_address: "Jakob-Haringer-Strasse 8/V".to_string(),
    ///     locality: "Salzburg".to_string(),
    ///     region: "Salzburg".to_string(),
    ///     postal_code: 5020.to_string(),
    ///
    ///     ..Default::default()
    /// };
    ///
    /// vcard.push(Property::new_adr(
    ///     address.clone(),
    ///     parameters!{"pref" => "1"}
    /// ));
    /// vcard.push(address);
    ///
    /// let json = json!([
    ///     "vcard",
    ///     [
    ///         ["version", {}, "text", "4.0"],
    ///
    ///         ["adr", {"pref": "1"}, "text", [
    ///             "",
    ///             "",
    ///             "Jakob-Haringer-Strasse 8/V",
    ///             "Salzburg",
    ///             "Salzburg",
    ///             "5020",
    ///             ""
    ///         ]],
    ///         ["adr", {}, "text", [
    ///             "",
    ///             "",
    ///             "Jakob-Haringer-Strasse 8/V",
    ///             "Salzburg",
    ///             "Salzburg",
    ///             "5020",
    ///             ""
    ///         ]]
    ///     ]
    /// ]);
    ///
    /// let parsed: Vcard = serde_json::from_value(json.clone())?;
    ///
    /// assert_eq!(serde_json::to_value(&vcard)?, json);
    /// assert_eq!(parsed, vcard);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_adr(address: Address, parameters: impl Into<Option<Parameters>>) -> Self {
        Self::new("adr", parameters, "text", address)
    }

    /// # Example
    /// /**
    /// ```rust
    /// # use vicardi::*;
    /// # use serde_json::json;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut vcard = Vcard::default();
    /// vcard.push(Property::new_org("Vicardi", None));
    /// vcard.push(Property::new_org(PropertyValue::Structured(vec!["Vicardi".into(), "Rust development".into()]), None));
    ///
    /// let json = json!([
    ///     "vcard",
    ///     [
    ///         ["version", {}, "text", "4.0"],
    ///         ["org", {}, "text", "Vicardi"],
    ///         ["org", {}, "text", ["Vicardi", "Rust development"]]
    ///     ]
    /// ]);
    ///
    /// let parsed: Vcard = serde_json::from_value(json.clone())?;
    ///
    /// assert_eq!(serde_json::to_value(&vcard)?, json);
    /// assert_eq!(parsed, vcard);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_org(
        org: impl Into<PropertyValue>,
        parameters: impl Into<Option<Parameters>>,
    ) -> Self {
        Self::new("org", parameters, "text", org)
    }

    /// # Example
    /// ```rust
    /// # use vicardi::*;
    /// # use serde_json::json;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut vcard = Vcard::default();
    /// vcard.push(Property::new_tel(Telephone::Voice, "+1-555-555-5555", None));
    ///
    /// let json = json!([
    ///     "vcard",
    ///     [
    ///         ["version", {}, "text", "4.0"],
    ///         ["tel", {"type": "voice"}, "uri", "tel:+1-555-555-5555"]
    ///     ]
    /// ]);
    ///
    /// let parsed: Vcard = serde_json::from_value(json.clone())?;
    ///
    /// assert_eq!(serde_json::to_value(&vcard)?, json);
    /// assert_eq!(parsed, vcard);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_tel(
        phone_type: impl Into<Telephone>,
        number: impl AsRef<str>,
        parameters: impl Into<Option<Parameters>>,
    ) -> Self {
        let mut parameters = parameters.into().unwrap_or_default();
        parameters.insert("type".into(), vec![phone_type.into().to_string()]);

        Self::new("tel", parameters, "uri", format!("tel:{}", number.as_ref()))
    }

    /// # Example
    /// ```rust
    /// # use vicardi::*;
    /// # use serde_json::json;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut vcard = Vcard::default();
    /// vcard.push(Property::new_email("vicardi@example.com", None));
    ///
    /// let json = json!([
    ///     "vcard",
    ///     [
    ///         ["version", {}, "text", "4.0"],
    ///         ["email", {}, "text", "vicardi@example.com"]
    ///     ]
    /// ]);
    ///
    /// let parsed: Vcard = serde_json::from_value(json.clone())?;
    ///
    /// assert_eq!(serde_json::to_value(&vcard)?, json);
    /// assert_eq!(parsed, vcard);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_email(email: impl ToString, parameters: impl Into<Option<Parameters>>) -> Self {
        Self::new("email", parameters, "text", email.to_string())
    }
}

impl<T> From<T> for PropertyValue
where
    T: ToString,
{
    fn from(value: T) -> Self {
        PropertyValue::String(value.to_string())
    }
}
