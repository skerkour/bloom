use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref VAT_FORMATS: HashMap<String, Regex> = {
        let mut m = HashMap::new();
        // Source: https://www.gov.uk/guidance/vat-eu-country-codes-vat-numbers-and-vat-in-other-languages
        // Austria
        m.insert("AT".to_string(), Regex::new(r"^ATU\d{8}$").unwrap());
        // Belgium
        m.insert("BE".to_string(), Regex::new(r"^BE0?\d{9, 10}$").unwrap());
        // Bulgaria
        m.insert("BG".to_string(), Regex::new(r"^BG\d{9,10}$").unwrap());
        // Croatia
        m.insert("HR".to_string(), Regex::new(r"^HR\d{11}$").unwrap());
        // Cyprus
        m.insert("CY".to_string(), Regex::new(r"^CY\d{8}[A-Z]$").unwrap());
        // Czech Republic
        m.insert("CZ".to_string(), Regex::new(r"^CZ\d{8,10}$").unwrap());
        // Denmark
        m.insert("DK".to_string(), Regex::new(r"^DK\d{8}$").unwrap());
        // Estonia
        m.insert("EE".to_string(), Regex::new(r"^EE\d{9}$").unwrap());
        // Finland
        m.insert("FI".to_string(), Regex::new(r"^FI\d{8}$").unwrap());
        // France
        m.insert("FR".to_string(), Regex::new(r"^FR[A-HJ-NP-Z0-9][A-HJ-NP-Z0-9]\d{9}$").unwrap());
        // Germany
        m.insert("DE".to_string(), Regex::new(r"^DE\d{9}$").unwrap());
        // Greece
        m.insert("EL".to_string(), Regex::new(r"^EL\d{9}$").unwrap());
        // Hungary
        m.insert("HU".to_string(), Regex::new(r"^HU\d{8}$").unwrap());
        // Ireland
        m.insert("IE".to_string(), Regex::new(r"^IE\d[A-Z0-9\+\*]\d{5}[A-Z]{1,2}$").unwrap());
        // Italy
        m.insert("IT".to_string(), Regex::new(r"^IT\d{11}$").unwrap());
        // Latvia
        m.insert("LV".to_string(), Regex::new(r"^LV\d{11}$").unwrap());
        // Lithuania
        m.insert("LT".to_string(), Regex::new(r"^LT(\d{9}|\d{12})$").unwrap());
        // Luxembourg
        m.insert("LU".to_string(), Regex::new(r"^LU\d{8}$").unwrap());
        // Malta
        m.insert("MT".to_string(), Regex::new(r"^MT\d{8}$").unwrap());
        // The Netherlands
        m.insert("NL".to_string(), Regex::new(r"^NL\d{9}B\d{2}$").unwrap());
        // Poland
        m.insert("PL".to_string(), Regex::new(r"^PL\d{10}$").unwrap());
        // Portugal
        m.insert("PT".to_string(), Regex::new(r"^PT\d{9}$").unwrap());
        // Romania
        m.insert("RO".to_string(), Regex::new(r"^RO\d{2,10}$").unwrap());
        // Slovak Republic
        m.insert("SK".to_string(), Regex::new(r"^SK\d{10}$").unwrap());
        // Slovenia
        m.insert("SI".to_string(), Regex::new(r"^SI\d{8}$").unwrap());
        // Spain
        m.insert("ES".to_string(), Regex::new(r"^ES[A-Z0-9]\d{7}[A-Z0-9]$").unwrap());
        // Sweden
        m.insert("SE".to_string(), Regex::new(r"^SE\d{10}01$").unwrap());

        m
    };
}

pub fn clean_vat_number(vat_number: &str) -> String {
    // TODO: maybe add 01 at the end of swedish code if len(vat_number) === 10?
    vat_number.to_uppercase().replace(" ", "").replace("-", "").to_string()
}

pub fn validate_format(vat_number: &str) -> bool {
    let cleaned = clean_vat_number(vat_number);
    match VAT_FORMATS.get(&cleaned[0..2]) {
        Some(re) => re.is_match(&cleaned),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{clean_vat_number, validate_format};

    #[test]
    fn test_validate_format_ok() {
        let numbers = vec![
            "ATU99999999",
            "BE1234567890",
            "BE0999999999",
            "BG1234567890",
            "HR12345678901",
            "CY12345678X",
            "CZ12345678",
            "CZ123456789",
            "CZ1234567890",
            "DK12345678",
            "EE123456789",
            "FI12345678",
            "FR12345678901",
            "FRX1234567890",
            "FR1X123456789",
            "FRXX123456789",
            "DE123456789",
            "EL123456789",
            "HU12345678",
            "IE1234567X",
            "IE1X23456X",
            "IE1234567XX",
            "IT12345678901",
            "LV12345678901",
            "LT123456789",
            "LT123456789012",
            "LU12345678",
            "MT12345678",
            "NL123456789B01",
            "PL1234567890",
            "PT123456789",
            // seriously romania
            "RO12",
            "RO123",
            "RO1234",
            "RO12345",
            "RO123456",
            "RO1234567",
            "RO12345678",
            "RO123456789",
            "RO1234567890",
            "SK1234567890",
            "SI12345678",
            "ESX12345678",
            "ES12345678X",
            "ESX1234567X",
            // TODO: add 01 if length is 10 for sweden?
            "SE123456789001",
        ];

        for number in numbers {
            assert!(validate_format(number));
        }
    }

    #[test]
    fn test_validate_format_unknown_country() {
        assert_eq!(validate_format("XX9980438"), false);
    }

    #[test]
    fn test_clean_vat_number() {
        let numbers = vec![("LU26375245", "LU26375245"), ("lu26375245", "LU26375245")];
        for (start, expected) in numbers {
            assert_eq!(clean_vat_number(start), expected.to_string());
        }
    }
}
