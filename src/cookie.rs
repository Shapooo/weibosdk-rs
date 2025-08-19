/// While expires parser of cookie crate is too strict, expires of weibo cookies parsing fails.
/// It must be re-implement to make the parsing succeed.
/// Some code snippts copied from https://github.com/rwf2/cookie-rs/blob/master/src/parse.rs
use std::collections::HashMap;

use log::{error, warn};
use reqwest_cookie_store::{CookieStore, RawCookie};
use serde::Deserialize;
use time::{OffsetDateTime, PrimitiveDateTime, macros::format_description};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Cookie {
    pub cookie: HashMap<String, String>,
}

impl TryFrom<Cookie> for CookieStore {
    type Error = Error;
    fn try_from(value: Cookie) -> std::result::Result<Self, Self::Error> {
        let mut cookie_store = CookieStore::new();
        for (domain, cookie) in value.cookie.iter() {
            let url =
                std::borrow::Cow::Borrowed("https://") + domain.strip_prefix('.').unwrap_or(domain);
            let request_url = url::Url::parse(&url)
                .map_err(|e| {
                    error!("{url} parse failed: {e}");
                })
                .unwrap();
            for cookie in cookie.lines().map(parse_cookie) {
                let _ = cookie_store
                    .insert_raw(&cookie?, &request_url)
                    .map_err(|e| {
                        warn!("cookie of {url} insert failed: {e}");
                    });
            }
        }

        Ok(cookie_store)
    }
}

fn parse_cookie<'a>(cookie_str: &'a str) -> Result<RawCookie<'a>> {
    let mut cookie = RawCookie::parse(cookie_str).map_err(|e| {
        error!("{cookie_str} parse failed: {e}");
        Error::DataConversionError(e.to_string())
    })?;

    if cookie.expires_datetime().is_none() {
        let expiry_date_str = cookie_str
            .split(';')
            .filter_map(|key_value| {
                key_value
                    .find('=')
                    .map(|i| (key_value[..i].trim(), key_value[(i + 1)..].trim()))
            })
            .find_map(|(key, value)| key.eq_ignore_ascii_case("expires").then_some(value));
        if let Some(expires) = expiry_date_str.map(parse_date) {
            cookie.set_expires(expires);
        }
    }

    Ok(cookie)
}

fn parse_date(ts: &str) -> Option<OffsetDateTime> {
    let fmt1 = format_description!(
        "[weekday repr:short], [day]-[month repr:short]-[year] [hour]:[minute]:[second] GMT"
    );
    let fmt2 = format_description!(
        "[weekday], [day]-[month repr:short]-[year] [hour]:[minute]:[second] GMT"
    );
    PrimitiveDateTime::parse(ts, &fmt1)
        .or_else(|_| PrimitiveDateTime::parse(ts, &fmt2))
        .map(|t| t.assume_utc())
        .map_err(|e| {
            error!("time {ts} parse failed {e}");
        })
        .ok()
}

#[cfg(test)]
mod local_tests {
    use std::path::Path;

    use reqwest_cookie_store::CookieStore;
    use serde_json::Value;

    use crate::cookie::Cookie;

    fn make_login_response() -> String {
        let res_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/login.json");
        let login_res = std::fs::read_to_string(res_path).unwrap();
        login_res
    }

    #[test]
    fn test_parse_cookie() {
        let s = make_login_response();
        let v: Value = serde_json::from_str(&s).unwrap();
        let map = v["cookie"]["cookie"].as_object().unwrap();
        for (_domain, cookie) in map {
            for cookie in cookie.as_str().unwrap().lines() {
                let _cookie = super::parse_cookie(cookie).unwrap();
            }
        }
    }

    #[test]
    fn test_cookie_to_cookie_store() {
        let s = make_login_response();
        let mut v: Value = serde_json::from_str(&s).unwrap();
        let cookie: Cookie = serde_json::from_value(v["cookie"].take()).unwrap();
        let _cookie_store: CookieStore = cookie.try_into().unwrap();
    }
}
