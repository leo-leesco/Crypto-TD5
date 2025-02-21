use std::{
    fs::{read, write, OpenOptions},
    io::Write,
};

use primitive_types::U256;
use TD5::{
    hash::sha512_mod_group_order,
    modulo::{add_modulo, mul_modulo},
    point::G,
    secret::secret_expand,
    Q,
};

fn sign(pk: [u8; 32], message: &[u8]) -> (U256, U256) {
    let (x, secret) = secret_expand(pk);

    let secret = secret.to_little_endian();
    let mut prefix_message: Vec<u8> = Vec::with_capacity(secret.len() + message.len());
    prefix_message.extend_from_slice(&secret);
    prefix_message.extend_from_slice(message);

    let r = sha512_mod_group_order(prefix_message);
    let r_point = (G * r).compress();
    let r_point_array = r_point.to_little_endian();
    let a = x.to_little_endian();
    let mut point_public_message: Vec<u8> =
        Vec::with_capacity(r_point_array.len() + a.len() + message.len());
    point_public_message.extend_from_slice(&r_point_array);
    point_public_message.extend_from_slice(&a);
    point_public_message.extend(message);

    let h = sha512_mod_group_order(point_public_message);
    let s = add_modulo(r, mul_modulo(x, h, Q), Q);
    (r_point, s)
}

fn main() {
    let mut args = std::env::args();
    let pk: [u8; 32] = read(
        args.nth(1)
            .expect("Please provide the path to the private key"),
    )
    .expect("Could not read the private key file")
    .try_into()
    .expect("Could not parse the private key as a 32-byte key");

    let message = read(
        args.nth(2)
            .expect("Please provide the path to the message file"),
    )
    .expect("Could not read the message file");

    let (rs, s) = sign(pk, &message);
    let rs = rs.to_little_endian();
    let s = s.to_little_endian();

    let sigfile = args.nth(3).expect("Please provide the path to the sigfile");
    write(&sigfile, rs).unwrap_or_else(|_| panic!("Could not write to {sigfile}"));
    let mut sigfile = OpenOptions::new().append(true).open(sigfile).unwrap();
    sigfile.write_all(&s).unwrap();
}
