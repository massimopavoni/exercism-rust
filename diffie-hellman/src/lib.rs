use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_mod(b_pub, a, p)
}

fn exp_mod(b: u64, e: u64, m: u64) -> u64 {
    let mut b = b as u128;
    let mut e = e;
    let mut r = 1;

    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m as u128;
        }
        b = (b * b) % m as u128;
        e /= 2;
    }

    r as u64
}
