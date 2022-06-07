use crate::models::{Location, Telephone, VCard, VElement};
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

    pub fn add_org(&mut self, name: Option<String>, unit: Option<String>) {
        self.add_vcard(
            "org".to_string(),
            HashMap::new(),
            "text".to_string(),
            VElement::Element(format!(
                "{} {}",
                name.unwrap_or_else(|| "".to_string()),
                unit.unwrap_or_else(|| "".to_string())
            )),
        )
    }

    pub fn add_address(&mut self, location: Location) {
        let mut properties: HashMap<String, String> = HashMap::new();

        properties.insert("cc".to_string(), "AT".to_string());

        // Array pn given order
        // the post office box;
        // the extended address (e.g., apartment or suite number);
        // the street address;
        // the locality (e.g., city);
        // the region (e.g., state or province);
        // the postal code;
        // the country name (full name);
        self.add_vcard(
            "adr".to_string(),
            properties,
            "text".to_string(),
            VElement::Array(vec![
                location.post_office_box.unwrap_or_else(|| "".to_string()),
                location.extended_address.unwrap_or_else(|| "".to_string()),
                location.street_address.unwrap_or_else(|| "".to_string()),
                location.locality.unwrap_or_else(|| "".to_string()),
                location.region.unwrap_or_else(|| "".to_string()),
                location.postal_code.unwrap_or_else(|| "".to_string()),
                location.country.unwrap_or_else(|| "".to_string()),
            ]),
        )
    }

    pub fn add_tel(&mut self, types: Telephone, number: String) {
        let mut properties: HashMap<String, String> = HashMap::new();

        match types {
            Telephone::Voice => {
                properties.insert("type".to_string(), "voice".to_string());
            }
            Telephone::Fax => {
                properties.insert("type".to_string(), "fax".to_string());
            }
        }

        self.add_vcard(
            "tel".to_string(),
            properties,
            "uri".to_string(),
            VElement::Element(format!("tel:{}", number)),
        )
    }

    pub fn add_email(&mut self, email: String) {
        self.add_vcard(
            "email".to_string(),
            HashMap::new(),
            "text".to_string(),
            VElement::Element(email),
        );
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
    use crate::{Location, VCardArray};

    #[test]
    fn test_fn() {
        let mut vcard = VCardArray::new();

        vcard.add_fn("John".to_string(), "Doe".to_string());

        let result =
            "[\"vcard\",[[\"version\",{},\"text\",\"4.0\"],[\"fn\",{},\"text\",\"John Doe\"]]]"
                .to_string();
        assert_eq!(vcard.to_json(false), result);
    }

    #[test]
    fn test_adr() {
        let mut vcard = VCardArray::new();

        vcard.add_address(Location {
            post_office_box: None,
            extended_address: None,
            street_address: Some("Jakob-Haringer-Strasse 8/V".to_string()),
            locality: Some("Salzburg".to_string()),
            region: Some("Salzburg".to_string()),
            postal_code: Some(5020.to_string()),
            country: None,
        });

        let result =
            "[\"vcard\",[[\"version\",{},\"text\",\"4.0\"],[\"adr\",{\"cc\":\"AT\"},\"text\",[\"\",\"\",\"Jakob-Haringer-Strasse 8/V\",\"Salzburg\",\"Salzburg\",\"5020\",\"\"]]]]"
                .to_string();
        assert_eq!(vcard.to_json(false), result);
    }

    #[test]
    fn test_email() {
        let mut vcard = VCardArray::new();

        vcard.add_email("sample@mail.com".to_string());

        let result =
            "[\"vcard\",[[\"version\",{},\"text\",\"4.0\"],[\"email\",{},\"text\",\"sample@mail.com\"]]]"
                .to_string();
        assert_eq!(vcard.to_json(false), result);
    }
}
