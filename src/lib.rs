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

    pub fn to_json(&self) -> String {
        let array: Vec<VCard> = vec![
            VCard::Element("vcard".to_string()),
            VCard::ElementArray(self.elements.clone()),
        ];

        serde_json::to_string_pretty(&array).unwrap()
    }
}

// pub fn vcardarray<F, U, C, K>() -> Vec<VCard> {
//     let mut result: Vec<VCard> = Vec::new();
//     let mut elements: Vec<Vec<VElement>> = Vec::new();
//
//     result.push(VCard::Element("vcard".to_string()));
//
//     elements.push(vec![
//         VElement::Element("version".to_string()),
//         VElement::Dictionary(HashMap::new()),
//         VElement::Element("text".to_string()),
//         VElement::Element("4.0".to_string()),
//     ]);
//
//     result.push(VCard::ElementArray(elements));
//
//     result
// }

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{VCardArray, VElement};

    #[test]
    fn sample_card() {
        let mut vcard = VCardArray::new();

        vcard.add_vcard(
            "version".to_string(),
            HashMap::new(),
            "text".to_string(),
            VElement::Element("4.0".to_string()),
        );

        println!("{}", vcard.to_json());

        let result = "[\n  \"vcard\",\n  [\n    [\n      \"version\",\n      {},\n      \"text\",\n      \"4.0\"\n    ]\n  ]\n]".to_string();
        assert_eq!(vcard.to_json(), result);
    }
}
