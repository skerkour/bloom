extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;
extern crate serde_urlencoded as urlencoded;

use rand::seq::SliceRandom;
use std::collections::HashMap;

use qs::Config;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Address {
    city: String,
    postcode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct QueryParams {
    id: u8,
    name: String,
    address: Address,
    phone: u32,
    user_ids: Vec<u8>,
}

fn main() {
    // Encodes as:
    // "user_ids%5B3%5D=4&user_ids%5B2%5D=3&address%5Bcity%5D=Carrot+City&\
    // id=42&address%5Bpostcode%5D=12345&name=Acme&user_ids%5B0%5D=1&\
    // phone=12345&user_ids%5B1%5D=2"
    let example_params = QueryParams {
        id: 42,
        name: "Acme".to_string(),
        phone: 12345,
        address: Address {
            city: "Carrot City".to_string(),
            postcode: "12345".to_string(),
        },
        user_ids: vec![1, 2, 3, 4],
    };

    // Naive approach: manually parameters in a map. Painful.
    let mut map = HashMap::<&str, &str>::new();
    map.insert("id", "42");
    map.insert("name", "Acme");
    map.insert("phone", "12345");
    map.insert("address[city]", "Carrot City");
    map.insert("address[postcode]", "12345");
    map.insert("user_ids[0]", "1");
    map.insert("user_ids[1]", "2");
    map.insert("user_ids[2]", "3");
    map.insert("user_ids[3]", "4");

    // Note this will be in some random order due to ordering of keys in map.
    let encoded = qs::to_string(&map).unwrap();
    println!("`serde_qs` to_string for map:\n\t{}", encoded);

    // In this form, can also simply use `serde_urlencoded`:
    let encoded = urlencoded::to_string(&map).unwrap();
    println!("`serde_urlencoded` to_string for map:\n\t{}", encoded);
    println!();

    // Given this encoded string, you can recover the original map
    // as a list of pairs using serde_urlencoded:
    let pairs: Vec<(String, String)> = urlencoded::from_str(&encoded).unwrap();
    println!("`serde_urlencoded` from_str to pairs:\n\t{:?}", pairs);

    // However, the best way is to use serde_qs to deserialize the entire thing
    // into a struct:
    //
    // (For this round trip to work, it's necessary to parse the query string
    // in non-strict mode, to allow parsing of url_encoded square brackets
    // in the key. See the lib.rs documentation for why).
    let qs_non_strict = Config::new(5, false);
    let params: QueryParams = qs_non_strict.deserialize_str(&encoded).unwrap();
    assert_eq!(params, example_params);
    println!("`serde_qs` from_str to struct:\n\t{:?}", params);

    // Similarly, we can serialize this structure using `serde_qs`:
    let encoded = qs::to_string(&params).unwrap();
    println!("`serde_qs` to_string for struct:\n\t{:?}", encoded);
    println!();

    // One nice feature is that this gives deterministic encodings:
    let encoded2 = qs::to_string(&params).unwrap();
    assert_eq!(encoded, encoded2);

    // An advantage of `serde_qs` for deserializing, is that it is robust
    // against different orderings of inputs:

    let mut inputs = vec![
        "id=42",
        "name=Acme",
        "phone=12345",
        "address[city]=Carrot+City",
        "address[postcode]=12345",
        "user_ids[0]=1",
        "user_ids[1]=2",
        "user_ids[2]=3",
        "user_ids[3]=4",
    ];

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let mut acc = String::new();
        inputs.shuffle(&mut rng);
        for input in &inputs {
            acc += input;
            acc += "&";
        }
        // remove trailing character
        acc.pop();
        let params: QueryParams = qs::from_str(&acc).unwrap();
        assert_eq!(params, example_params);
    }

    // By default, `serde_qs` uses arrays with indices to denote position.
    // However, if omitted, will use input order:
    let encoded = "id=42&name=Acme&phone=12345&address[city]=Carrot+City&\
                   address[postcode]=12345&\
                   user_ids[]=1&\
                   user_ids[]=2&\
                   user_ids[]=3&\
                   user_ids[]=4";
    let params: QueryParams = qs::from_str(encoded).unwrap();
    assert_eq!(params, example_params);

    // Indices do not necessarily need to be continuous:
    let encoded = "id=42&name=Acme&phone=12345&address[city]=Carrot+City&\
                   address[postcode]=12345&\
                   user_ids[1]=2&\
                   user_ids[0]=1&\
                   user_ids[12]=3&\
                   user_ids[512]=4";
    let params: QueryParams = qs::from_str(encoded).unwrap();
    assert_eq!(params, example_params);

    // Enums are now fully supported! Most formats should work with varying
    // results.
    #[derive(Deserialize, Debug, PartialEq, Serialize)]
    enum AdjTaggedEnum {
        A,
        B(bool),
        S(String),
        V { id: u8, v: String },
    }

    #[derive(Deserialize, Debug, PartialEq, Serialize)]
    struct EnumQuery {
        e: AdjTaggedEnum,
    }

    let example_params = EnumQuery {
        e: AdjTaggedEnum::B(false),
    };
    // encodes as:
    //   "e[B]=false"
    let encoded = qs::to_string(&example_params).unwrap();
    println!("`serde_qs` to_string for enum:\n\t{:?}", encoded);
    let params: EnumQuery = qs::from_str(&encoded).unwrap();
    println!("`serde_qs` from_str for enum:\n\t{:?}", params);
    println!();

    let example_params = EnumQuery {
        e: AdjTaggedEnum::A,
    };
    // encodes as:
    //   "e=A"
    let encoded = qs::to_string(&example_params).unwrap();
    println!("`serde_qs` to_string for enum:\n\t{:?}", encoded);
    let params: EnumQuery = qs::from_str(&encoded).unwrap();
    println!("`serde_qs` from_str for enum:\n\t{:?}", params);
    println!();
}
