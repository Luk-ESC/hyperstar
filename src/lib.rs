use num::bigint::BigUint;
use num::{Integer, Zero};

mod macros;
mod tests;

/// Calculate the value of a digit at place `place` in a number of base `base`.<br>
/// `place` is counting from the right, starting at 1.
fn value_of_digit(digit: &BigUint, base: &BigUint, place: usize) -> BigUint {
    if digit.is_zero() {
        return biguint!(0);
    }
    base.pow((place - 1) as _) * digit
}

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

pub struct Number {
    whole: Vec<BigUint>,
    decimal: Vec<BigUint>,
    base: BigUint,
}

impl Number {
    fn decimal_to_base(&self, to_base: &BigUint, up_to: usize) -> Vec<BigUint> {
        let mut output = vec![];

        let digit_count = self.decimal.len();
        let mut value = value_of_digits(&self.decimal, &self.base);
        while  !value.is_zero() && output.len() < up_to {
            let mut digits = to_digit_arr(value * to_base, &self.base);

            if digits.len() < digit_count {
                println!("hi");
                digits.reverse();
                digits.extend(std::iter::repeat_with(|| biguint!(0)).take(digit_count - digits.len()));
                digits.reverse();
            }

            let (whole, decimal_part) = digits.split_at(digits.len() - digit_count);
            value = value_of_digits(decimal_part, &self.base);

            output.push(value_of_digits(whole, &self.base));
        }

        output
    }


    pub fn to_base(&self, base: BigUint, up_to: usize) -> Self {
        let whole_value = value_of_digits(&self.whole, &self.base);
        let whole = to_digit_arr(whole_value, &base);

        let decimal = self.decimal_to_base(&base, up_to);
        Number {
            whole,
            decimal,
            base,
        }
    }


    fn new(value: &str) -> Self {
        let (whole, decimal) = value.split_once('.').unwrap();
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
}