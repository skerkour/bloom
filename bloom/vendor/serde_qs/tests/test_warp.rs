#![cfg(feature = "warp")]

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;
extern crate warp_framework as warp;

use qs::Config as QsConfig;
use serde::de::Error;
use warp::{http::StatusCode, Filter};

fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
{
    let s = <&str as serde::Deserialize>::deserialize(deserializer)?;
    S::from_str(&s).map_err(|_| D::Error::custom("could not parse string"))
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Query {
    foo: u64,
    bars: Vec<u64>,
    #[serde(flatten)]
    common: CommonParams,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct CommonParams {
    #[serde(deserialize_with = "from_str")]
    limit: u64,
    #[serde(deserialize_with = "from_str")]
    offset: u64,
    #[serde(deserialize_with = "from_str")]
    remaining: bool,
}

#[test]
fn test_default_error_handler() {
    futures::executor::block_on(async {
        let filter = qs::warp::query::<Query>(QsConfig::default())
            .map(|_| "")
            .recover(qs::warp::recover_fn);

        let resp = warp::test::request().path("/test").reply(&filter).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    })
}

#[test]
fn test_composite_querystring_extractor() {
    futures::executor::block_on(async {
        let filter = qs::warp::query::<Query>(QsConfig::default());
        let s = warp::test::request()
            .path("/test?foo=1&bars[]=0&bars[]=1&limit=100&offset=50&remaining=true")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(s.foo, 1);
        assert_eq!(s.bars, vec![0, 1]);
        assert_eq!(s.common.limit, 100);
        assert_eq!(s.common.offset, 50);
        assert_eq!(s.common.remaining, true);
    })
}

#[test]
fn test_default_qs_config() {
    futures::executor::block_on(async {
        let filter = qs::warp::query::<Query>(QsConfig::default())
            .map(|_| "")
            .recover(qs::warp::recover_fn);

        let resp = warp::test::request()
            .path("/test?foo=1&bars%5B%5D=3&limit=100&offset=50&remaining=true")
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    })
}

#[test]
fn test_custom_qs_config() {
    futures::executor::block_on(async {
        let filter = qs::warp::query::<Query>(QsConfig::new(5, false));
        let s = warp::test::request()
            .path("/test?foo=1&bars%5B%5D=3&limit=100&offset=50&remaining=true")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(s.foo, 1);
        assert_eq!(s.bars, vec![3]);
        assert_eq!(s.common.limit, 100);
        assert_eq!(s.common.offset, 50);
        assert_eq!(s.common.remaining, true);
    })
}
