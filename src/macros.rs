/// Convert any primitive integer to a BigUint.
/// Created because BigUint doesn't implement From for i32 and
/// you have to specify the type for integer literals.
///
/// # Example
/// ```rust
/// use num_bigint::BigUint;
/// use hyperstar::biguint;
/// let x = biguint!(10);
/// assert_eq!(x, BigUint::from(10u8));
///
/// let y = biguint!(u16::MAX);
/// assert_eq!(y, BigUint::from(u16::MAX));
/// ```
#[macro_export]
macro_rules! biguint {
    ($val:expr) => {
        ::num_bigint::BigUint::from($val as u128)
    };
}


/// Create an array of BigUints
///
/// # Example
/// ```rust
/// use num_bigint::BigUint;
/// use hyperstar::biguint_arr;
///
/// let x = biguint_arr!(1, 2, 3);
/// assert_eq!(x, [BigUint::from(1u8), BigUint::from(2u8), BigUint::from(3u8)])
/// ```
#[macro_export]
macro_rules! biguint_arr {
    ($($val:expr),+) => {
        [$(
            ::num_bigint::BigUint::from($val as u128),
        )+]
    };

    () => { [] }
}
