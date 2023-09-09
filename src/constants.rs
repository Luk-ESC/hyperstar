use num::BigUint;
use std::sync::LazyLock;

pub static DECIMAL: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(10u8));
pub static ZERO: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(0u8));
