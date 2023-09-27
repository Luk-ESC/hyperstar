use num::bigint::BigUint;
use num::integer::Integer;
use num::Zero;
use std::fmt::{Debug, Formatter, Write};
use std::str::FromStr;

mod macros;
mod tests;

// convert base10 str to vec<Biguint>
fn convert(x: &str) -> Vec<BigUint> {
    x.as_bytes().iter().map(|&x| biguint!(x - b'0')).collect()
}

/// Calculate the value of a whole number in base `base`
#[inline(always)]
pub fn value_of_digits(digits: &[BigUint], base: &BigUint) -> BigUint {
    if digits.is_empty() {
        return biguint!(0);
    }

    if digits.len() == 1 {
        return digits[0].clone();
    }

    let mut place = biguint!(1);
    let mut sum = biguint!(0);

    for digit in digits.iter().rev() {
        sum += place.clone() * digit;
        place *= base;
    }
    sum
}

/// Convert a number to an array of digits in `base`.
fn to_digit_arr(mut number: BigUint, base: &BigUint) -> Vec<BigUint> {
    let mut digits = Vec::with_capacity(10);

    while &number >= base {
        let (div, rest) = number.div_mod_floor(base);

        number = div;
        digits.push(rest);
    }

    digits.push(number);
    digits.reverse();
    digits
}

/// A Number represented as a Vector of digits associated with a base with functionality for converting between different bases. <bR>
///
/// The documentation often mentions "whole" and "decimal" parts.
/// This might be confusing because the "decimal" part of a number here has nothing to do with
/// Base 10. <bR>
/// For example, for `0b101.111`, `101` is the whole part and `111` is the decimal part.
///
#[derive(Eq, PartialEq)]
pub struct Number {
    whole: Vec<BigUint>,
    left_div: BigUint,
    right_div: BigUint,
    base: BigUint,
    negative: bool,
}

// TODO doc
fn expand(mut left: BigUint, right: BigUint, base: &BigUint) -> (Vec<BigUint>, usize) {
    let mut left_arr = to_digit_arr(left.clone(), &base);
    let mut added = 0usize;
    // Expansion: 11 / 100 -> 110 / 100 added = 1
    let right_val = right;
    while left < right_val { // TODO: perf
        left_arr.push(biguint!(0));
        added += 1;
        left *= base;
    }

    (left_arr, added)
}

impl Number {
    pub fn get_whole_part(&self) -> &[BigUint] {
        &self.whole
    }

    pub fn get_base(&self) -> &BigUint {
        &self.base
    }

    pub fn get_lossy_decimal(&self, precision: usize) -> Vec<BigUint> {
        let (mut left, added) = expand(self.left_div.clone(), self.right_div.clone(), self.get_base());
        let right = to_digit_arr(self.right_div.clone(), self.get_base());


        let mut digits = vec![];
        loop {
            let mut to_take = right.len();

            if digits.len() > precision {
                return digits;
            }
            let (chunk, div) = loop {
                let chunk = &left[0..to_take];
                let chunk = value_of_digits(chunk, self.get_base());
                let div = self.right_div.clone();
                if chunk >= div {
                    break (chunk, div);
                }
                to_take += 1;
            };

            let (a, b) = chunk.div_mod_floor(&div);
            digits.push(a);
            if b.is_zero() {
                return digits;
            }
            let (a, b) = expand(b, div, self.get_base());
            left = a;
            digits.extend(std::iter::repeat(biguint!(0)).take(b - 1));
        }


        digits
    }

    /// Convert self to base. `up_to` specifies the digits of precision, in the case
    /// where self is not accurately representable in the other base.
    /// It is guaranteed that the decimal part of the Number returned has a length that is smaller
    /// or equal to `up_to`.
    ///
    /// # Example
    /// ```rust
    /// use hyperstar::{biguint, biguint_arr, Number};
    /// let x = Number::new("16.25").to_base(biguint!(4));
    ///
    /// assert_eq!(x.get_whole_part(), biguint_arr!(1, 0, 0));
    /// assert_eq!(x.get_lossy_decimal(100), biguint_arr!(1));
    /// ```
    pub fn to_base(&self, base: BigUint) -> Self {
        assert!(base >= biguint!(2));
        let whole_value = value_of_digits(&self.whole, &self.base);
        let whole = to_digit_arr(whole_value, &base);

        Number {
            whole,
            left_div: self.left_div.clone(),
            right_div: self.right_div.clone(),
            base,
            negative: self.negative,
        }
    }

    /// Construct a number from a readable &str, written in base 10.
    /// Because this is only meant to create constants, it could panic or result in weird behaviour
    /// when an invalid string is passed in.
    ///
    /// # Example
    /// ```rust
    /// use hyperstar::{biguint, biguint_arr, Number};
    ///
    /// let x = Number::new("1234.42");
    /// assert_eq!(x.get_base(), &biguint!(10));
    /// assert_eq!(x.get_whole_part(), &biguint_arr!(1, 2, 3, 4));
    /// assert_eq!(x.get_lossy_decimal(100), &biguint_arr!(4, 2));
    /// ```
    pub fn new(value: &str) -> Self {
        let (whole, decimal) = value.split_once('.').unwrap_or((value, "0"));

        let mut negative = false;
        let whole = if whole.starts_with('-') {
            negative = true;
            &whole[1..]
        } else if whole.is_empty() {
            "0"
        } else {
            whole
        };

        let whole = convert(whole);

        let right_div = biguint!(10).pow(decimal.len() as u32);
        let left_div = BigUint::from_str(decimal).unwrap();

        Number {
            base: biguint!(10),
            whole,
            right_div,
            left_div,
            negative,
        }
    }

    pub fn new_from_frac(negative: bool, whole: Option<Vec<BigUint>>, s: &str) -> Number {
        let whole = whole.unwrap_or_default();

        let (left_div, right_div) = s.split_once("/").unwrap();
        let (left_div, right_div) = (convert(left_div), convert(right_div));
        let (left_div, right_div) = (
            value_of_digits(&left_div, &biguint!(10)),
            value_of_digits(&right_div, &biguint!(10)),
        );

        Number {
            base: biguint!(10),
            whole,
            right_div,
            left_div,
            negative,
        }
    }

    /// Construct a number from digit arrays.
    ///
    /// # Example
    /// ```rust
    /// use hyperstar::{biguint, biguint_arr, Number};
    ///
    /// let whole = biguint_arr!(0).to_vec();
    /// let decimal = biguint_arr!(1, 0, 1).to_vec();
    /// let base = biguint!(2);
    ///
    /// let x = Number::from(whole.clone(), decimal.clone(), base.clone(), false);
    ///
    /// assert_eq!(x.get_base(), &base);
    /// assert_eq!(x.get_whole_part(), &whole);
    /// assert_eq!(x.get_lossy_decimal(100), decimal);
    /// ```
    pub fn from(whole: Vec<BigUint>, decimal: Vec<BigUint>, base: BigUint, negative: bool) -> Self {
        assert!(base >= biguint!(2));
        for i in whole.iter().chain(decimal.iter()) {
            assert!(i < &base);
        }

        let right_div = base.pow(decimal.len() as u32);
        let left_div = value_of_digits(&decimal, &base);

        Self {
            whole,
            left_div,
            right_div,
            base,
            negative,
        }
    }
}

fn normalize_to(base: &BigUint, mut right_div: BigUint, mut left_div: BigUint) -> (BigUint, BigUint) {
    let y = right_div.gcd(&left_div);
    right_div /= y.clone();
    left_div /= y;



    (right_div, left_div)
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.left_div, self.right_div))
    }
}
