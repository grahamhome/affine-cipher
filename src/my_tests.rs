#[cfg(test)]
mod my_tests {
    use crate::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(20, 7), 1);
    }

    #[test]
    fn euclid_extended_works() {
        assert_eq!(euclid_extended(60, 48, None, None, None, None), Ok((12, 0, -1)));
        assert_eq!(euclid_extended(123, 456, None, None, None, None), Ok((3, -63, 17)));
    }

    #[test]
    fn char_to_int_works() {
        assert_eq!(char_to_int('A'), 0);
        assert_eq!(char_to_int('Z'), 25);
        assert_eq!(char_to_int('a'), 0);
        assert_eq!(char_to_int('z'), 25);
    }

    #[test]
    fn int_to_char_works() {
        assert_eq!(int_to_char(0), 'a');
        assert_eq!(int_to_char(25), 'z');
    }
}
