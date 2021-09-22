pub fn test_string(s: &str, n: usize) -> usize {
    let times_to_repeat_string = (n / s.len()) + n % s.len();
    let mut infinite_string = "".to_string();
    for _ in 0..times_to_repeat_string {
        infinite_string = format!("{}{}", infinite_string, s);
    }

    let mut number_of_as = 0;
    for char in infinite_string.chars().take(n) {
        if char == 'a' {
            number_of_as = number_of_as + 1;
        }
    }

    number_of_as
}

#[cfg(test)]
mod tests {
    use crate::hackerrank::test_string;

    #[test]
    fn it_works() {
        // assert_eq!(test_string("aba", 10), 7);
        assert_eq!(test_string("a", 1_000_000_000_000), 7);
    }
}
