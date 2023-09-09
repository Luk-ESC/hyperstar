#[macro_export]
macro_rules! biguint {
    ($val:expr) => {
        ::num::bigint::BigUint::from($val as u64)
    };
}

#[macro_export]
macro_rules! biguint_arr {
    [$($val:expr),+] => {
        [$(
            ::num::bigint::BigUint::from($val as u64),
        )+]
    }
}
