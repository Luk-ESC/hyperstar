use std::fmt::{Debug, Formatter};
use num_bigint::BigUint;
use num_integer::Integer;

mod macros;
mod tests;

/// Calculate the value of a digit at place `place` in a number of base `base`.<br>
/// `place` is counting from the right, starting at 1.
fn value_of_digit(digit: &BigUint, base: &BigUint, place: usize) -> BigUint {
    if digit == &biguint!(0) {
        return biguint!(0);
    }
    base.pow((place - 1) as _) * digit
}

/// Calculate the value of a whole number in base `base`
fn value_of_digits(digits: &[BigUint], base: &BigUint) -> BigUint {
    digits
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            let place = digits.len() - i;
            value_of_digit(digit, base, place)
        })
        .sum()
}

/// Convert a number to an array of digits in `base`.
fn to_digit_arr(mut number: BigUint, base: &BigUint) -> Vec<BigUint> {
    let mut digits = Vec::new();

    while &number >= base {

        let (div, rest) = number.div_mod_floor(base);

        number = div;
        digits.insert(0, rest);
    }

    digits.insert(0, number);
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
    decimal: Vec<BigUint>,
    base: BigUint,
}

impl Number {
    pub fn get_decimal_part(&self) -> &[BigUint] {
        &self.decimal
    }
    pub fn get_whole_part(&self) -> &[BigUint]{
        &self.whole
    }

    pub fn get_base(&self) -> &BigUint {
        &self.base
    }

    /// Convert decimal part of self to different base. `up_to` specifies how many digits to calculate.
    fn decimal_to_base(&self, to_base: &BigUint, up_to: usize) -> Vec<BigUint> {
        let mut output = vec![];

        let digit_count = self.decimal.len();

        let mut value = value_of_digits(&self.decimal, &self.base);

        if value == biguint!(0){
            return vec![biguint!(0)];
        }

        while value != biguint!(0) && output.len() < up_to {
            let mut digits = to_digit_arr(value * to_base, &self.base);

            while digits.len() < digit_count {
                digits.insert(0, biguint!(0));
            }

            let (whole, decimal_part) = digits.split_at(digits.len() - digit_count);
            value = value_of_digits(decimal_part, &self.base);

            output.push(value_of_digits(whole, &self.base));
        }

        output
    }

    /// Convert self to base. `up_to` specifies the digits of precision, in the case
    /// where self is not accurately representable in the other base.
    /// It is guaranteed that the decimal part of the Number returned has a length that is smaller
    /// or equal to `up_to`.
    ///
    /// # Example
    /// ```rust
    /// use hyperstar::{biguint, biguint_arr, Number};
    /// let x = Number::new("16.25").to_base(biguint!(4), 100);
    ///
    /// assert_eq!(x.get_whole_part(), biguint_arr!(1, 0, 0));
    /// assert_eq!(x.get_decimal_part(), biguint_arr!(1));
    /// ```
    pub fn to_base(&self, base: BigUint, up_to: usize) -> Self {
        assert!(base >= biguint!(2));
        let whole_value = value_of_digits(&self.whole, &self.base);
        let whole = to_digit_arr(whole_value, &base);

        let decimal = self.decimal_to_base(&base, up_to);
        Number {
            whole,
            decimal,
            base,
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
    /// assert_eq!(x.get_decimal_part(), &biguint_arr!(4, 2));
    /// ```
    pub fn new(value: &str) -> Self {
        let (whole, decimal) = value.split_once('.').unwrap_or((value, "0"));
        let whole = if whole.is_empty() { "0" } else { whole };
        fn convert(x: &str) -> Vec<BigUint> {
            x.as_bytes()
                .iter()
                .map(|&x| biguint!(x - b'0'))
                .collect()
        }

        let (whole, decimal) = (convert(whole), convert(decimal));
        Number {
            base: biguint!(10),
            whole,
            decimal,
        }
    }

    /// Construct a number from digit arrays.
    ///
    /// # Example
    /// ```rust
    /// use hyperstar::{biguint, biguint_arr, Number};
    ///
    /// let whole = biguint_arr!(16, 32, 64).to_vec();
    /// let decimal = biguint_arr!(1, 0, 1).to_vec();
    /// let base = biguint!(100);
    ///
    /// let x = Number::from(whole.clone(), decimal.clone(), base.clone());
    ///
    /// assert_eq!(x.get_base(), &base);
    /// assert_eq!(x.get_whole_part(), &whole);
    /// assert_eq!(x.get_decimal_part(), &decimal);
    /// ```
    pub fn from(whole: Vec<BigUint>, decimal: Vec<BigUint>, base: BigUint) -> Self {
        assert!(base >= biguint!(2));
        for i in whole.iter().chain(decimal.iter()) {
            assert!(i < &base);
        }

        Self { whole, decimal, base}
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = format!("{:?}.{:?}", self.whole, self.decimal);
        f.write_str(&x)
    }
}