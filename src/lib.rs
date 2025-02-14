#![allow(non_snake_case)]

use primitive_types::U256;

const P: U256 = U256([
    0xffffffffffffffed,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);
/// this is actually (A+2)/4
const A: U256 = U256([121666, 0, 0, 0]);
pub mod conversion;
pub mod montgomery;
