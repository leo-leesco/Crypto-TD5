use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use primitive_types::U256;

use crate::{
    modulo::{add_modulo, mul_modulo, sub_modulo},
    D, P,
};

use super::{Point, ZERO};

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

impl Add<Self> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
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
        Self {
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
        *self = ZERO;
        for i in (0..rhs.bits()).rev() {
            if rhs.bit(i) {
                *self += p;
            }
            p += p;
        }
    }
}

impl Mul<U256> for Point {
    type Output = Self;
    /// this is not a constant time scalar multiplication
    fn mul(self, rhs: U256) -> Self::Output {
        let mut p = self;
        let mut q = ZERO;
        for i in (0..rhs.bits()).rev() {
            if rhs.bit(i) {
                q += p;
            }
            p += p;
        }
        q
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (mul_modulo(self.X, other.Z, P) == mul_modulo(other.X, self.Z, P))
            && (mul_modulo(self.Y, other.Z, P) == mul_modulo(other.Y, self.Z, P))
    }
}

/// aP+bQ
pub fn multiexp(a: U256, p: Point, b: U256, q: Point) -> Point {
    let t = (ZERO, p, q, p + q);
    (0..256).rev().fold(ZERO, |acc, i| {
        acc * U256([2, 0, 0, 0]) + {
            match (a.bit(i), b.bit(i)) {
                (false, false) => t.0,
                (true, false) => t.1,
                (false, true) => t.2,
                (true, true) => t.3,
            }
        }
    })
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            X: P - self.X,
            Z: P - self.Z,
            ..self
        }
    }
}
