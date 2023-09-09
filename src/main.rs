#![feature(lazy_cell)]

use num::bigint::BigUint;
use num::{Integer, Zero};
use crate::constants::ZERO;
mod constants;

macro_rules! biguint {
    ($val:expr) => {
        ::num::bigint::BigUint::from($val as u64)
    };

    [$($val:expr),+] => {
        [$(
            ::num::bigint::BigUint::from($val as u64),
        )+]
    }
}

/// Calculate the value of a digit at place `place` in a number of base `base`.<br>
/// `place` is counting from the right, starting at 1.
///
/// # Example
/// ```rust
/// // Number: 123, base 10
/// // 1 is at place 3, counting from the right
/// // That digit has a value of 100
///
/// let value = calculate_value(biguint!(1), biguint!(10), biguint!(3));
/// assert_eq!(value, biguint!(100))
fn calculate_value(digit: BigUint, base: BigUint, place: usize) -> BigUint{
    if digit.is_zero() {
        return ZERO.clone();
    }
    base.pow((place - 1) as _) * digit
}

/// Convert a whole number to `to_base`.
pub fn convert_whole_to_base(digits: Vec<BigUint>, from_base: BigUint, to_base: BigUint) -> Vec<BigUint>{
    let mut real_number = biguint!(0);

    let digit_count = digits.len();
    for (i, v) in digits.into_iter().enumerate(){
        let place = digit_count - i;
        real_number += calculate_value(v, from_base.clone(), place);
    }

    to_digit_arr(real_number, to_base)
}

/// Convert a number to an array of digits in `base`.
fn to_digit_arr(mut number: BigUint, base: BigUint) -> Vec<BigUint> {
    let mut digits = vec![];

    while number >= base {
        println!("{number}");
        let (div, rest) = number.div_mod_floor(&base);

        println!("{div}, {rest}");
        number = div;
        digits.push(rest);
    }
    digits.push(number);
    digits.reverse();
    digits
}


fn main() {
}

#[cfg(test)]
mod testing {
    use crate::*;
    use crate::constants::*;

    #[test]
    pub(crate) fn calculate_value_() {
        let digit = ZERO.clone();
        let place = usize::MAX;
        let base = biguint!(u16::MAX);
        assert_eq!(calculate_value(digit, base, place), ZERO.clone());

        let digit = biguint!(9);
        let place = 1;
        let base = DECIMAL.clone();
        assert_eq!(calculate_value(digit, base, place), biguint!(9));

        let digit = biguint!(8);
        let place = 2;
        let base = DECIMAL.clone();
        assert_eq!(calculate_value(digit, base, place), biguint!(80));
    }

    #[test]
    pub(crate) fn to_digit_arr_() {
        let arr = to_digit_arr(biguint!(0b1010101), biguint!(2));
        assert_eq!(arr, biguint![1, 0, 1, 0, 1, 0, 1]);


        let arr = to_digit_arr(biguint!(123456789), biguint!(10));
        assert_eq!(arr, biguint![1, 2, 3, 4, 5, 6, 7, 8, 9]);


        let arr = to_digit_arr(biguint!(100), biguint!(10));
        assert_eq!(arr, biguint![1, 0, 0]);
    }

    #[test]
    fn convert_whole_to_base_() {

        let nums = biguint![0, 1, u32::MAX, u8::MAX];
        for number in nums {
            let binary = to_digit_arr(number.clone(), biguint!(2));
            let decimal = to_digit_arr(number, biguint!(10));
            let converted = convert_whole_to_base(binary, biguint!(2), biguint!(10));
            assert_eq!(decimal, converted);
        }

    }
}