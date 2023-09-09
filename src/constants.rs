use std::sync::LazyLock;
use num::BigUint;

pub static DECIMAL: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(10u8));
pub static ZERO: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(0u8));