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
        VCardArray { elements: vec![] }
    }

    pub fn add_vcard(
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
    use crate::{VCardArray, VElement};
    use std::collections::HashMap;

    #[test]
    fn sample_card() {
        let mut vcard = VCardArray::new();

        vcard.add_vcard(
            "version".to_string(),
            HashMap::new(),
            "text".to_string(),
            VElement::Element("4.0".to_string()),
        );

        println!("{}", vcard.to_json(true));

        let result = "[\"vcard\",[[\"version\",{},\"text\",\"4.0\"]]]".to_string();
        assert_eq!(vcard.to_json(false), result);
    }
}
