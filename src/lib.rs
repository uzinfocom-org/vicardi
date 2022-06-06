use crate::models::{VCard, VElement};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod models;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VCardArray {
    elements: Vec<Vec<VElement>>,
}

impl VCardArray {
    pub fn new() -> Self {
        VCardArray {
            elements: vec![vec![
                VElement::Element("version".to_string()),
                VElement::Dictionary(HashMap::new()),
                VElement::Element("text".to_string()),
                VElement::Element("4.0".to_string()),
            ]],
        }
    }

    fn add_vcard(
        &mut self,
        category: String,
        properties: HashMap<String, String>,
        types: String,
        value: VElement,
    ) {
        self.elements.push(vec![
            VElement::Element(category),
            VElement::Dictionary(properties),
            VElement::Element(types),
            value,
        ]);
    }

    pub fn add_fn(&mut self, name: String, surname: String) {
        self.add_vcard(
            "fn".to_string(),
            HashMap::new(),
            "text".to_string(),
            VElement::Element(format!("{} {}", name, surname)),
        )
    }

    pub fn add_org(&mut self, name: String, unit: String) {
        self.add_vcard(
            "org".to_string(),
            HashMap::new(),
            "text".to_string(),
            VElement::Element(format!("{} {}", name, unit)),
        )
    }

    pub fn add_address(&mut self, street: String, city: String, country: String) {
        let mut properties: HashMap<String, String> = HashMap::new();

        properties.insert("cc".to_string(), "AT".to_string());

        self.add_vcard(
            "adr".to_string(),
            properties,
            "text".to_string(),
            VElement::Array(vec![street, city, country]),
        )
    }

    pub fn add_tel(&mut self, types: &str, number: String) {
        let mut properties: HashMap<String, String> = HashMap::new();

        match types {
            "v" => {
                properties.insert("type".to_string(), "voice".to_string());
            }
            "f" => {
                properties.insert("type".to_string(), "fax".to_string());
            }
            _ => {
                properties.insert("type".to_string(), "undefined".to_string());
            }
        }

        self.add_vcard(
            "tel".to_string(),
            properties,
            "uri".to_string(),
            VElement::Element(format!("tel:{}", number)),
        )
    }

    pub fn to_json(&self, pretty: bool) -> String {
        let array: Vec<VCard> = vec![
            VCard::Element("vcard".to_string()),
            VCard::ElementArray(self.elements.clone()),
        ];

        match pretty {
            true => serde_json::to_string_pretty(&array).unwrap(),
            false => serde_json::to_string(&array).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::VCardArray;

    #[test]
    fn sample_card() {
        let mut vcard = VCardArray::new();

        vcard.add_fn("John".to_string(), "Doe".to_string());

        println!("{}", vcard.to_json(true));

        let result =
            "[\"vcard\",[[\"version\",{},\"text\",\"4.0\"],[\"fn\",{},\"text\",\"John Doe\"]]]"
                .to_string();
        assert_eq!(vcard.to_json(false), result);
    }
}
