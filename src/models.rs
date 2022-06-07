use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum VElement {
    Element(String),
    Dictionary(HashMap<String, String>),
    Array(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum VCard {
    Element(String),
    ElementArray(Vec<Vec<VElement>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VCardField {
    Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub post_office_box: Option<String>,
    pub extended_address: Option<String>,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Telephone {
    Fax,
    Voice,
}
