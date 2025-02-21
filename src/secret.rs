use primitive_types::U256;

use crate::hash::sha512;

pub fn secret_expand(secret: [u8; 32]) -> (U256, U256) {
    let hash = sha512(secret);
    let mut a = U256::from_little_endian(&hash[..32]);
    a &= (U256::one() << 254) - U256([8, 0, 0, 0]);
    a |= U256::one() << 254;
    (a, U256::from_little_endian(&hash[32..]))
}
