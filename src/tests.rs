#[cfg(test)]
mod testing {
    use crate::*;

    #[test]
    pub(crate) fn to_digit_arr_() {
        let arr = to_digit_arr(biguint!(0b1010101), &biguint!(2));
        assert_eq!(arr, biguint_arr![1, 0, 1, 0, 1, 0, 1]);

        let arr = to_digit_arr(biguint!(123456789), &biguint!(10));
        assert_eq!(arr, biguint_arr![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let arr = to_digit_arr(biguint!(100), &biguint!(10));
        assert_eq!(arr, biguint_arr![1, 0, 0]);
    }

    #[test]
    fn repeating() {
        let a = Number::new("1.67").to_base(biguint!(16), 500);
        let a = a.to_base(biguint!(10), 5);
        assert_eq!(&a.decimal, &biguint_arr!(6, 6, 9, 9, 9));
    }

    #[test]
    fn normal() {
        let a = Number::new("0.5").to_base(biguint!(2), 500);
        assert_eq!(&a.decimal, &biguint_arr!(1));

        let a = Number::new("0.2").to_base(biguint!(5), 10);
        assert_eq!(&a.decimal, &biguint_arr!(1));
    }

    #[test]
    fn limits() {
        let mut x = String::new();

        x.extend(std::iter::repeat("69420").take(100));
        x.push('.');
        x.extend(std::iter::repeat("69420").take(100));

        let a = Number::new(&x);
        let x = a.to_base(biguint!(u16::MAX), 100);
        let y = x.to_base(biguint!(10), 20);
        assert_eq!(a.to_base(biguint!(10), 20), y);
    }

    #[test]
    fn base_42(){
        let x = Number::new("42.0").to_base(biguint!(43), 1000);
        assert_eq!(x.whole, biguint_arr!(42));
        assert_eq!(x.decimal, biguint_arr!(0));
    }
}
