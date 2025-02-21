use primitive_types::U256;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    X: U256,
    Y: U256,
    Z: U256,
    T: U256,
}

const ZERO: Point = Point {
    X: U256::zero(),
    Y: U256::one(),
    Z: U256::one(),
    T: U256::zero(),
};

/// 4/5 mod p = 0x6666666666666666 6666666666666666 6666666666666666 6666666666666658
const G_Y: U256 = U256([
    0x6666666666666658,
    0x6666666666666666,
    0x6666666666666666,
    0x6666666666666666,
]);
/// recover_x(G_Y,sign=0) = 0x216936d3cd6e53fe c0a4e231fdd6dc5c 692cc7609525a7b2 c9562d608f25d51a
const G_X: U256 = U256([
    0xc9562d608f25d51a,
    0x692cc7609525a7b2,
    0xc0a4e231fdd6dc5c,
    0x216936d3cd6e53fe,
]);
/// G_X*G_Y mod p = 0x67875f0fd78b7665 66ea4e8e64abe37d 20f09f80775152f5 6dde8ab3a5b7dda3
pub const G: Point = Point {
    X: G_X,
    Y: G_Y,
    Z: U256::one(),
    T: U256([
        0x6dde8ab3a5b7dda3,
        0x20f09f80775152f5,
        0x66ea4e8e64abe37d,
        0x67875f0fd78b7665,
    ]),
};

mod compression;
pub mod operations;
