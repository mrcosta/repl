use std::io;

macro_rules! read_int(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Something went wrong when reading the input.");
        line.trim().parse::<usize>().expect("Something went wrong when parsing the input to an usize value")
    }}
);

fn main() {
    let n = read_int!();

    fizz_buzz(n);
}

pub fn fizz_buzz(n: usize) {
    for i in 1..=n {
        match ((i % 3), (i % 5)) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fizz_buzz;

    #[test]
    fn test_case_1() {
        let expected_output = "\
        1
        2
        Fizz
        4
        Buzz
        Fizz
        7
        8
        Fizz
        Buzz
        11
        Fizz
        13
        14
        FizzBuzz"
            .to_string();
        assert_eq!(fizz_buzz(15), expected_output);
    }
}
