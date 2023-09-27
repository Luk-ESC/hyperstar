use num::BigUint;

#[cfg(test)]
mod testing {
    use crate::*;

    #[test]
    fn base5(){
        let a = Number::new("0.28").to_base(biguint!(5));
        assert_eq!(&a.get_lossy_decimal(10), &biguint_arr!(1, 2));
    }

    #[test]
    fn yoooooo(){
        let a = Number::from(biguint_arr!(0).to_vec(), biguint_arr!(1, 0, 1).to_vec(), biguint!(2), false);
        assert_eq!(&a.get_lossy_decimal(100), &biguint_arr!(1, 0, 1));
    }

    #[test]
    fn yooooo0o(){
        let a = to_digit_arr(biguint!(8), &biguint!(2));
        assert_eq!(&a, &biguint_arr!(1, 0, 0, 0));
    }

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
        let a = Number::new("1.3").to_base(biguint!(11));
        let a = a.to_base(biguint!(10));
        assert_eq!(&a.get_lossy_decimal(10), &biguint_arr!(3));
    }

    #[test]
    fn normal() {
        let a = Number::new("0.5").to_base(biguint!(2));
        assert_eq!(&a.get_lossy_decimal(10), &biguint_arr!(1));
    }
    #[test]
    fn yooo() {
        let a = Number::new("0.2").to_base(biguint!(5));
        assert_eq!(&a.get_lossy_decimal(10), &biguint_arr!(1));
    }

    #[test]
    fn limits() {
        let mut x = String::new();

        x.extend(std::iter::repeat("69420").take(100));
        x.push('.');
        x.extend(std::iter::repeat("69420").take(100));

        let a = Number::new(&x);
        let x = a.to_base(biguint!(u16::MAX));
        let y = x.to_base(biguint!(10));
        assert_eq!(a.to_base(biguint!(10)), y);
    }

    /*#[test]
    fn base_42() {
        let x = Number::new("42.0").to_base(biguint!(43));
        assert_eq!(x.whole, biguint_arr!(42));
        assert_eq!(x.get_lossy_decimal(10), biguint_arr!(0));
    }*/

    #[test]
    fn test1() {
        let x = Number::new("0.1").to_base(biguint!(2));

        let x = x.to_base(biguint!(10));
        assert_eq!(x.get_lossy_decimal(10), biguint_arr!(1));
    }
}
