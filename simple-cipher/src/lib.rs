use rand::Rng;

pub fn encode(key: &str, pt: &str) -> Option<String> {
    shift_cipher(key, pt, |p, k| ((p - b'a' + k - b'a') % 26 + b'a') as char)
}

pub fn decode(key: &str, ct: &str) -> Option<String> {
    shift_cipher(key, ct, |c, k| ((26 + c - k) % 26 + b'a') as char)
}

pub fn encode_random(pt: &str) -> (String, String) {
    let mut rng = rand::rng();

    let key: String = (0..128)
        .map(|_| (rng.random_range(..26_u8) + b'a') as char)
        .collect();

    let ct = encode(&key, pt);

    (key, ct.unwrap_or_default())
}

fn shift_cipher<OP>(key: &str, input: &str, encdec: OP) -> Option<String>
where
    OP: Fn(u8, u8) -> char,
{
    if key.is_empty() {
        return None;
    }

    let mut out = String::new();

    for (c, k) in input.bytes().zip(key.bytes().cycle()) {
        if !k.is_ascii_lowercase() {
            return None;
        }

        out.push(encdec(c, k));
    }

    Some(out)
}
