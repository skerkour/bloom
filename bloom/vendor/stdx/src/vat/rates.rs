use super::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const RATES_NUMBER: usize = 27;

lazy_static! {
    // very basic and only contains the main rate
    static ref VAT_RATES: HashMap<String, u8> = {
        let mut m = HashMap::new();
        // Austria
        m.insert("AT".to_string(), 20);
        // Belgium
        m.insert("BE".to_string(), 21);
        // Bulgaria
        m.insert("BG".to_string(), 20);
        // Cyprus
        m.insert("CY".to_string(), 19);
        // Czech Republic
        m.insert("CZ".to_string(), 21);
        // Germany
        m.insert("DE".to_string(), 19);
        // Denmark
        m.insert("DK".to_string(), 25);
        // Estonia
        m.insert("EE".to_string(), 20);
        // Spain
        m.insert("ES".to_string(), 21);
        // Finland
        m.insert("FI".to_string(), 24);
        // France
        m.insert("FR".to_string(), 20);
        // Greece
        m.insert("GR".to_string(), 24);
        // Croatia
        m.insert("HR".to_string(), 25);
        // Hungary
        m.insert("HU".to_string(), 27);
        // Ireland
        m.insert("IE".to_string(), 23);
        // Italy
        m.insert("IT".to_string(), 22);
        // Lithuania
        m.insert("LT".to_string(), 21);
        // Luxembourg
        m.insert("LU".to_string(), 17);
        // Latvia
        m.insert("LV".to_string(), 21);
        // Malta
        m.insert("MT".to_string(), 18);
        // The Netherlands
        m.insert("NL".to_string(), 21);
        // Poland
        m.insert("PL".to_string(), 23);
        // Portugal
        m.insert("PT".to_string(), 23);
        // Romania
        m.insert("RO".to_string(), 19);
        // Sweden
        m.insert("SE".to_string(), 25);
        // Slovenia
        m.insert("SI".to_string(), 22);
        // Slovak Republic
        m.insert("SK".to_string(), 20);

        m
    };
}

pub fn get_rate(country: &str) -> Result<u8, Error> {
    match VAT_RATES.get(&country.to_uppercase()) {
        Some(r) => Ok(*r),
        None => Err(Error::InvalidCountry(country.to_string())),
    }
}

pub fn get_all_rates() -> HashMap<String, u8> {
    VAT_RATES.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_get_rate_eu_country() {
        let res = get_rate("fr");
        assert!(res.is_ok());
        assert_eq!(20, res.unwrap());
    }

    #[test]
    fn test_cannot_get_rate_non_eu_country() {
        let res = get_rate("us");
        assert!(res.is_err());
    }

    #[test]
    fn number_of_vat_rates_countries() {
        let rates = get_all_rates();
        assert_eq!(RATES_NUMBER, rates.len());
    }
}
