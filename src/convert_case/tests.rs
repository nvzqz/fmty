#![cfg(test)]

use test_strategy::proptest;

use super::*;

mod to_ascii_uppercase {
    use super::*;

    #[proptest]
    fn debug(s: String) {
        let expected = format!("{:?}", s).to_ascii_uppercase();
        let result = format!("{:?}", to_ascii_uppercase(s));

        assert_eq!(expected, result);
    }

    #[proptest]
    fn display(s: String) {
        let expected = s.to_ascii_uppercase();
        let result = to_ascii_uppercase(s).to_string();

        assert_eq!(expected, result);
    }
}

mod to_ascii_lowercase {
    use super::*;

    #[proptest]
    fn debug(s: String) {
        let expected = format!("{:?}", s).to_ascii_lowercase();
        let result = format!("{:?}", to_ascii_lowercase(s));

        assert_eq!(expected, result);
    }

    #[proptest]
    fn display(s: String) {
        let expected = s.to_ascii_lowercase();
        let result = to_ascii_lowercase(s).to_string();

        assert_eq!(expected, result);
    }
}
