#[cfg(test)]
mod my_tests {
    use crate::*;

    #[test]
    fn gcdx_works() {
        assert_eq!(gcdx(60, 48), (12, 1, -1));
        assert_eq!(gcdx(123, 456), (3, -63, 17));
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
