use std::{
    fs::{read, write, OpenOptions},
    io::Write,
};

use primitive_types::U256;
use TD5::{
    hash::sha512_mod_group_order,
    point::{operations::multiexp, Point, G},
    Q,
};

fn verify(pk: [u8; 32], message: &[u8], signature: [u8; 64]) -> bool {
    let a = Point::decompress(U256::from_little_endian(&pk));
    let r = Point::decompress(U256::from_little_endian(&signature[..32]));
    let s = U256::from_little_endian(&signature[32..]);
    assert!(s < Q);

    let mut point_public_message: Vec<u8> =
        Vec::with_capacity(signature[..32].len() + pk.len() + message.len());
    point_public_message.extend_from_slice(&signature[..32]);
    point_public_message.extend_from_slice(&pk);
    point_public_message.extend(message);

    let h = sha512_mod_group_order(point_public_message);
    r == multiexp(s, G, Q - h, a) // since we did not implement negative
                                  // multiplication, we complement to Q, which is the group order
}

fn main() {
    let mut args = std::env::args();
    let pk: [u8; 32] = read(
        args.nth(1)
            .expect("Please provide the path to the public key"),
    )
    .expect("Could not read the public key file")
    .try_into()
    .expect("Could not parse the public key as a 32-byte key");

    let message = read(
        args.nth(2)
            .expect("Please provide the path to the message file"),
    )
    .expect("Could not read the message file");

    let sigfile = read(args.nth(3).expect("Please provide the path to the sigfile"))
        .expect("Could not read the sigfile")
        .try_into()
        .expect("Could not parse the sigfile as a 64-byte signature");

    if verify(pk, &message, sigfile) {
        println!("ACCEPT");
    } else {
        println!("REJECT");
    }
}
