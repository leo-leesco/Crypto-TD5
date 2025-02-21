use primitive_types::U256;
use sha2::{
    digest::{
        consts::{B0, B1},
        generic_array::GenericArray,
        typenum::{UInt, UTerm},
    },
    Digest, Sha512,
};

use crate::Q;

type Hash =
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>>;

pub fn sha512<T>(secret: T) -> Hash
where
    T: AsRef<[u8]>,
{
    let mut hasher = Sha512::new();
    hasher.update(secret);
    hasher.finalize()
}

pub fn sha512_mod_group_order<T>(secret: T) -> U256
where
    T: AsRef<[u8]>,
{
    U256::from_little_endian(&sha512(secret)) % Q
}
