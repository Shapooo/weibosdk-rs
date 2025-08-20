use std::borrow::Cow;

use serde::{Deserialize, Deserializer};
use serde_json::{Value, json};
use sha2::{Digest, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::params::*;

pub fn get_current_timestamp_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

pub(crate) fn generate_s(uid: &str, from: &str) -> String {
    let pin = "CypCHG2kSlRkdvr2RG1QF8b2lCWXl7k7";
    generate_s_(uid, pin, from)
}

pub(crate) fn build_common_params() -> Value {
    json!({
        "c": PARAM_C,
        "from": FROM,
        "source": SOURCE,
        "lang": LANG,
        "locale": LOCALE,
        "wm": WM,
        "ua": UA,
    })
}

fn generate_s_(uid: &str, pin: &str, from: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(pin);
    hasher.update(uid);
    hasher.update(from);
    let hash1 = hasher.finalize_reset();
    let hash1: Vec<_> = format!("{hash1:x}").chars().collect();
    let hash2 = Sha512::digest(from);
    let hash2: Vec<_> = hash2
        .into_iter()
        .flat_map(|n| {
            let low_nibble = n & 0x0f;
            let high_nibble = n >> 4;
            vec![high_nibble, low_nibble]
        })
        .collect();
    let mut i = 0u8;
    let mut res = String::new();
    for _ in 0..8 {
        i += hash2[i as usize];
        res.push(hash1[i as usize]);
    }
    res
}

pub fn deserialize_str_num<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Either<'a> {
        Str(Cow<'a, str>),
        Num(i32),
    }
    match Either::deserialize(deserializer)? {
        Either::Str(s) => s.parse().map_err(|e| serde::de::Error::custom(e)),
        Either::Num(n) => Ok(n),
    }
}

#[cfg(test)]
mod local_tests {
    use super::*;

    #[test]
    fn generate_s_test() {
        let from = "12DC195010";
        assert_eq!(generate_s("1219658392".into(), from), "fb111111");
        assert_eq!(generate_s("1054595560".into(), from), "23777777");
        assert_eq!(generate_s("1229101630".into(), from), "37222222");
        assert_eq!(generate_s("1494639172".into(), from), "77999999");
        assert_eq!(generate_s("1568849308".into(), from), "7ceeeeee");
        assert_eq!(generate_s("1927972896".into(), from), "92888888");
        assert_eq!(generate_s("1683934114".into(), from), "b8888888");
        assert_eq!(generate_s("1982981009".into(), from), "f5666666");
    }
}
