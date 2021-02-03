extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;

#[test]
fn double_encoding_keys() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Human {
        #[serde(rename = "full name")]
        name: String,
    }

    let human = Human {
        name: "John Doe".to_string(),
    };

    let encoded = serde_qs::to_string(&human).unwrap();
    print!("{}", encoded);
    assert_eq!(serde_qs::from_str::<Human>(&encoded).unwrap(), human);
}
