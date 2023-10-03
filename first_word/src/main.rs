fn main() {}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}

#[cfg(test)]
mod first_word_tests {
    use crate::first_word;

    #[test]
    fn empty_string() {
        assert_eq!(first_word(&String::from("")), "");
    }

    #[test]
    fn big_word() {
        assert_eq!(
            first_word(&String::from("dsncweiuhiuciuriwudendnciuewndewun")),
            "dsncweiuhiuciuriwudendnciuewndewun"
        );
    }

    #[test]
    fn two_word() {
        assert_eq!(first_word(&String::from("two words")), "two");
    }
}
