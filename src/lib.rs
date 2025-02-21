#![allow(non_snake_case)]

use primitive_types::U256;

/// p = 2**255 - 19
const P: U256 = U256([
    0xffffffffffffffed,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);

/// compared to the formula given in https://datatracker.ietf.org/doc/html/rfc8032#section-6
/// d = modp_inv(121666) - 1 (mod p)
///   = 0x52036cee2b6ffe73 8cc740797779e898 00700a4d4141d8ab 75eb4dca135978a4
const D: U256 = U256([
    0x75eb4dca135978a4,
    0x00700a4d4141d8ab,
    0x8cc740797779e898,
    0x52036cee2b6ffe73,
]);

/// q = 2**252 + 27742317777372353535851937790883648493
///   = 0x1000000000000000 0000000000000000 14def9dea2f79cd6 5812631a5cf5d3ed
pub const Q: U256 = U256([
    0x5812631a5cf5d3ed,
    0x14def9dea2f79cd6,
    0,
    0x1000000000000000,
]);

/// inverse of -1 mod p = 0x2b8324804fc1df0b 2b4d00993dfbd7a7 2f431806ad2fe478 c4ee1b274a0ea0b0
const INV_M1: U256 = U256([
    0xc4ee1b274a0ea0b0,
    0x2f431806ad2fe478,
    0x2b4d00993dfbd7a7,
    0x2b8324804fc1df0b,
]);

pub mod hash;
pub mod modulo;
pub mod point;
pub mod secret;
