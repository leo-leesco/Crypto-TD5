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

mod compression;
mod operations;
