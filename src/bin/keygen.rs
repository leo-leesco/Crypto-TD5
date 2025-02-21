use std::fs::write;

use primitive_types::U256;
use rand::Rng;
use sha2::{Digest, Sha512};
use TD5::point::G;

fn secret_expand(secret: [u8; 32]) -> (U256, U256) {
    let mut hasher = Sha512::new();
    hasher.update(secret);
    let hash = hasher.finalize();
    let mut a = U256::from_little_endian(&hash[..32]);
    a &= (U256::one() << 254) - U256([8, 0, 0, 0]);
    a |= U256::one() << 254;
    (a, U256::from_little_endian(&hash[32..]))
}

fn secret_to_public(secret: [u8; 32]) -> U256 {
    let (a, _) = secret_expand(secret);
    (G * a).compress()
}

fn main() {
    let secret = rand::rng().random();
    write("prefix.sk", secret).expect("Could not write the secret key to prefix.sk");
    write("prefix.pk", secret_to_public(secret).to_little_endian())
        .expect("Could not write the secret key to prefix.pk");
}
