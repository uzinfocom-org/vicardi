use std::collections::HashMap;
use crate::models::{VCard, VCardArray, VElement};

pub mod models;

pub fn vcardarray() -> VCardArray {
    let mut result: Vec<VCard> = VCardArray::new();
    let mut elements: Vec<Vec<VElement>> = Vec::new();

    result.push(VCard::VElement("vcard".to_string()));

    elements.push(vec![
        VElement::Element("version".to_string()),
        VElement::Dictionary(HashMap::new()),
        VElement::Element("text".to_string()),
        VElement::Element("4.0".to_string()),
    ]);

    result.push(VCard::VElementArray(elements));

    result
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
