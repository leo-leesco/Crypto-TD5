use primitive_types::{U256, U512};

/// (a+b)%p
pub fn add_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    U256::try_from((a + b) % p).unwrap()
}

/// (a*b)%p
pub fn mul_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    U256::try_from((a * b) % p).unwrap()
}

/// (a-b)%p
pub fn sub_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    if a > b {
        U256::try_from((a - b) % p).unwrap()
    } else {
        U256::try_from(((a + p) - b) % p).unwrap()
    }
}

/// (x^n)%p
pub fn pow_modulo(mut x: U256, mut n: U256, p: U256) -> U256 {
    let mut result = U256::one();
    while !n.is_zero() {
        if !(n & U256::one()).is_zero() {
            result = mul_modulo(result, x, p);
        }
        x = mul_modulo(x, x, p);
        n >>= 1;
    }
    result
}

pub fn inv_modulo(x: U256, p: U256) -> U256 {
    pow_modulo(x, p - 2, p)
}
