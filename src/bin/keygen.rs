use primitive_types::U256;
use TD4::{
    conversion::{clamp, sanitize},
    montgomery::Point,
};

fn main() {
    let mut args = std::env::args();
    let m = clamp(
        U256::from_str_radix(&args.nth(1).expect("Please provide m"), 16)
            .expect("Could not parse m as a 64-character hexadecimal string"),
    );
    let u = sanitize({
        if let Some(u) = &args.nth(2) {
            // DH key exchange
            U256::from_str_radix(u, 16)
                .expect("Could not parse m as a 64-character hexadecimal string")
        } else {
            // DH public key generation
            U256([9, 0, 0, 0])
        }
    });
    println!("{:0>64X}", (Point::new(u) * m).get());
}
