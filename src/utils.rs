use sha2::{Digest, Sha512};

pub(crate) fn generate_s(uid: &str, from: &str) -> String {
    let pin = "CypCHG2kSlRkdvr2RG1QF8b2lCWXl7k7";
    generate_s_(uid, pin, from)
}

fn generate_s_(uid: &str, pin: &str, from: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(pin);
    hasher.update(uid);
    hasher.update(from);
    let hash1 = hasher.finalize_reset();
    let hash1: Vec<_> = format!("{:x}", hash1).chars().collect();
    // hasher.update(pin);
    // let hash2 = hasher.finalize();
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

#[cfg(test)]
mod utils {
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
