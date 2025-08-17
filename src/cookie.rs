/// While expires parser of cookie crate is too strict, expires of weibo cookies parsing fails.
/// It must be re-implement to make the parsing succeed.
/// Some code snippts copied from https://github.com/rwf2/cookie-rs/blob/master/src/parse.rs
use std::collections::HashMap;

use cookie::SameSite;
use log::error;
use reqwest_cookie_store::{CookieStore, RawCookie};
use serde::Deserialize;
use time::{Duration, OffsetDateTime, macros::format_description};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Cookie {
    pub cookie: HashMap<String, String>,
}

impl TryFrom<Cookie> for CookieStore {
    type Error = Error;
    fn try_from(value: Cookie) -> std::result::Result<Self, Self::Error> {
        let mut cookie_store = CookieStore::new(None);
        for (domain, cookie) in value.cookie.iter() {
            let cookie = parse_cookie(cookie)?;
            let url =
                std::borrow::Cow::Borrowed("https://") + domain.strip_prefix('.').unwrap_or(domain);
            let request_url = url::Url::parse(&url)
                .map_err(|e| {
                    error!("{url} parse failed: {e}");
                })
                .unwrap();
            cookie_store
                .insert_raw(&cookie, &request_url)
                .map_err(|e| error!("cookie of {url} insert failed: {e}"))
                .unwrap();
        }

        Ok(cookie_store)
    }
}

fn parse_cookie<'a>(cookie: &'a str) -> Result<RawCookie<'a>> {
    let mut attributes = cookie.split(';');

    // Determine the name = val.
    let key_value = attributes
        .next()
        .expect("first str::split().next() returns Some");
    let (name, value) = match key_value.find('=') {
        Some(i) => (key_value[..i].trim(), key_value[(i + 1)..].trim()),
        None => return Err(Error::DataConversionError(format!("Error::MissingPair",))),
    };

    if name.is_empty() {
        return Err(Error::DataConversionError(
            "ParseError::EmptyName".to_string(),
        ));
    }

    let mut cookie = RawCookie::build((name, value));

    for attr in attributes {
        let (key, value) = match attr.find('=') {
            Some(i) => (attr[..i].trim(), Some(attr[(i + 1)..].trim())),
            None => (attr.trim(), None),
        };

        match (&*key.to_ascii_lowercase(), value) {
            ("secure", _) => cookie = cookie.secure(true),
            ("httponly", _) => cookie = cookie.http_only(true),
            ("max-age", Some(mut v)) => {
                let max_age = {
                    let is_negative = v.starts_with('-');
                    if is_negative {
                        v = &v[1..];
                    }

                    if !v.chars().all(|d| d.is_digit(10)) {
                        continue;
                    }

                    // From RFC 6265 5.2.2: neg values indicate that the earliest
                    // expiration should be used, so set the max age to 0 seconds.
                    if is_negative {
                        Duration::ZERO
                    } else {
                        v.parse::<i64>()
                            .map(Duration::seconds)
                            .unwrap_or_else(|_| Duration::seconds(i64::max_value()))
                    }
                };
                cookie = cookie.max_age(max_age);
            }
            ("domain", Some(d)) if !d.is_empty() => cookie = cookie.domain(d),
            ("path", Some(v)) => cookie = cookie.path(v),
            ("samesite", Some(v)) => {
                if v.eq_ignore_ascii_case("strict") {
                    // cookie.same_site = Some(SameSite::Strict);
                    cookie = cookie.same_site(SameSite::Strict);
                } else if v.eq_ignore_ascii_case("lax") {
                    cookie = cookie.same_site(SameSite::Lax);
                } else if v.eq_ignore_ascii_case("none") {
                    cookie = cookie.same_site(SameSite::None);
                } else {
                    // We do nothing here, for now. When/if the `SameSite`
                    // attribute becomes standard, the spec says that we should
                    // ignore this cookie, i.e, fail to parse it, when an
                    // invalid value is passed in. The draft is at
                    // http://httpwg.org/http-extensions/draft-ietf-httpbis-cookie-same-site.html.
                }
            }
            ("partitioned", _) => cookie = cookie.partitioned(true),
            ("expires", Some(v)) => {
                let format = format_description!(
                    "[weekday], [day]-[month repr:short]-[year] [hour]:[minute]:[second] GMT"
                );
                let time = OffsetDateTime::parse(v, &format);
                if let Ok(time) = time {
                    cookie = cookie.expires(time);
                }
            }
            _ => {
                // We're going to be permissive here. If we have no idea what
                // this is, then it's something nonstandard. We're not going to
                // store it (because it's not compliant), but we're also not
                // going to emit an error.
            }
        }
    }

    Ok(cookie.build())
}

#[cfg(test)]
mod local_tests {
    use std::path::Path;

    use reqwest_cookie_store::CookieStore;
    use serde_json::Value;

    use crate::cookie::Cookie;

    fn make_login_response() -> String {
        let res_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/login.json");
        let login_res = std::io::read_to_string(std::fs::File::open(res_path).unwrap()).unwrap();
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
