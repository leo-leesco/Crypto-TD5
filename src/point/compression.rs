use primitive_types::U256;

use crate::{
    modulo::{add_modulo, inv_modulo, mul_modulo, pow_modulo, sub_modulo},
    D, INV_M1, P,
};

use super::Point;

fn recover_x(y: U256, sign: bool) -> U256 {
    assert!(y < P);
    let y_squared = mul_modulo(y, y, P);
    let x_squared = mul_modulo(
        sub_modulo(y_squared, U256::one(), P),
        inv_modulo(add_modulo(mul_modulo(D, y_squared, P), U256::one(), P), P),
        P,
    );
    assert!(!sign && x_squared != U256::zero());

    if x_squared == U256::zero() {
        U256::zero()
    } else {
        let mut x = pow_modulo(x_squared, (P + 3) / 8, P);
        if mul_modulo(x, x, P) != x_squared {
            x = mul_modulo(x, INV_M1, P);
        }
        assert_eq!(mul_modulo(x, x, P), x_squared);
        if sign == x.bit(0) {
            x
        } else {
            sub_modulo(P, x, P)
        }
    }
}

impl Point {
    fn compress(self) -> U256 {
        let zinv = inv_modulo(self.Z, P);
        let x = mul_modulo(self.X, zinv, P);
        let y = mul_modulo(self.Y, zinv, P);
        y | ((x & U256::one()) << 255)
    }

    fn decompress(mut y: U256) -> Self {
        let sign = y >> 255 == U256::one();
        y &= (U256::one() << 255) - U256::one();
        let x = recover_x(y, sign);
        Point {
            X: x,
            Y: y,
            Z: U256::one(),
            T: mul_modulo(x, y, P),
        }
    }
}
