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
