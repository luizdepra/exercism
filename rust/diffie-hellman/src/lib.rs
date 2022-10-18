use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(p as u128, g as u128, a as u128) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(p as u128, b_pub as u128, a as u128) as u64
}

fn modular_pow(p: u128, g: u128, a: u128) -> u128 {
    let mut value = 1u128;
    for _ in 0..a {
        value = (value * g) % p;
    }

    value
}
