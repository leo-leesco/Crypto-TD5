use std::{env::args, fs::write};

use primitive_types::U256;
use rand::Rng;
use TD5::{hash::sha512, point::G};

fn secret_to_public(secret: [u8; 32]) -> U256 {
    let (a, _) = secret_expand(secret);
    (G * a).compress()
}

fn main() {
    let prefix = args()
        .nth(1)
        .expect("Please provide a filename to write to <filename>.sk and <filename>.pk");
    let secret = rand::rng().random();
    write(format!("{prefix}.sk"), secret).expect("Could not write the secret key to prefix.sk");
    write(
        format!("{prefix}.pk"),
        secret_to_public(secret).to_little_endian(),
    )
    .unwrap_or_else(|_| panic!("Could not write the secret key to {prefix}.pk"));
}
