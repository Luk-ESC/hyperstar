# Hyperstar
This program converts decimal numbers represented as lists of digits between different bases.

## Example
```rust
use hyperstar::{biguint, biguint_arr, Number};

let digits_of_precision = 1000;
let base = biguint!(4);
let x = Number::new("16.25").to_base(base, digits_of_precision);
    
assert_eq!(x.get_whole_part(), biguint_arr!(1, 0, 0));
assert_eq!(x.get_decimal_part(), biguint_arr!(1));
```
