use std::ops::{AddAssign, MulAssign};

use primitive_types::{U256, U512};

use crate::{D, P};

/// (a+b)%p
fn add_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    U256::try_from((a + b) % p).unwrap()
}

/// (a*b)%p
fn mul_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    U256::try_from((a * b) % p).unwrap()
}

/// (a-b)%p
fn sub_modulo(a: U256, b: U256, p: U256) -> U256 {
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

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, rhs: Self) {
        let a = mul_modulo(
            sub_modulo(self.Y, self.X, P),
            sub_modulo(rhs.Y, rhs.X, P),
            P,
        );
        let b = mul_modulo(
            add_modulo(self.Y, self.X, P),
            add_modulo(rhs.Y, rhs.X, P),
            P,
        );
        let c = mul_modulo(
            mul_modulo(U256([2, 0, 0, 0]), D, P),
            mul_modulo(self.T, rhs.T, P),
            P,
        );
        let d = mul_modulo(
            mul_modulo(U256([2, 0, 0, 0]), D, P),
            mul_modulo(self.Z, rhs.Z, P),
            P,
        );
        let e = sub_modulo(b, a, P);
        let f = sub_modulo(d, c, P);
        let g = add_modulo(b, a, P);
        let h = add_modulo(d, c, P);
        *self = Self {
            X: mul_modulo(e, f, P),
            Y: mul_modulo(g, h, P),
            Z: mul_modulo(f, g, P),
            T: mul_modulo(e, h, P),
        }
    }
}

impl MulAssign<U256> for Point {
    /// this is not a constant time scalar multiplication
    fn mul_assign(&mut self, rhs: U256) {
        let mut p = *self;
        for i in (0..rhs.bits()).rev() {
            if rhs.bit(i) {
                *self += p;
            }
            p += p;
        }
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (mul_modulo(self.X, other.Z, P) == mul_modulo(other.X, self.Z, P))
            && (mul_modulo(self.Y, other.Z, P) == mul_modulo(other.Y, self.Z, P))
    }
}
