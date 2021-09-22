use std::io;

macro_rules! read_int(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Something went wrong when reading the input.");
        line.trim().parse::<usize>().expect("Something went wrong when parsing the input to an usize value")
    }}
);

macro_rules! read_line(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Something went wrong when reading the input.");
        String::from(line.trim())
    }}
);

fn main() {
    let s = read_line!();
    let n = read_int!();

    println!("{}", test_string(&s, n));
}

pub fn test_string(s: &str, n: usize) -> usize {
    let times_to_repeat_string = n / s.len();
    let remaining_number_of_chars_to_count = n % s.len();

    let number_of_as_in_string = s.matches('a').count();
    let mut total_number_of_as = number_of_as_in_string * times_to_repeat_string;

    if remaining_number_of_chars_to_count != 0 {
        let reduced_string: String = s.chars().take(remaining_number_of_chars_to_count).collect();
        total_number_of_as = total_number_of_as + reduced_string.matches('a').count();
    }

    total_number_of_as
}

#[cfg(test)]
mod tests {
    use crate::test_string;

    #[test]
    fn test_case_1() {
        assert_eq!(test_string("aba", 10), 7);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(test_string("a", 1_000_000_000_000), 1000000000000);
    }
}
