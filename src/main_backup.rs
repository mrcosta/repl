#![allow(dead_code, unused_imports, unused_must_use)]

#[macro_use]
extern crate maplit;

use crate::closure_with_result_errors::{closure_with_result, closure_with_result_other};
use crate::filter::test_filter_map;
use crate::group_by::{group_by_example, group_by_second_example};
use crate::hackerrank::test_string;
use crate::merge_join_by::{merge_join_by_example, merge_join_by_example_with_sorted};
use crate::option::option;
use crate::reference::reference_test;
use crate::sorted_enum::print_sorted_enum;
use crate::threads::{
    thread_that_dont_wait_and_finish_after_main, thread_that_waits_and_finish_before_main,
};
use crate::vectors::test_sorted_vectors;
use std::io::{self, Read};

mod closure_with_result_errors;
mod filter;
mod group_by;
mod hackerrank;
mod merge_join_by;
mod option;
mod reference;
mod serde_deserialization;
mod sorted_enum;
mod threads;
mod vectors;

// fn main() {
// reference_test();
// group_by_second_example();
// group_by_example();
// closure_with_result_other();
// option();
// thread_that_dont_wait_and_finish_after_main();
// thread_that_waits_and_finish_before_main();
// print_sorted_enum();
// test_filter_map();
// merge_join_by_example();
// test_sorted_vectors();
// merge_join_by_example_with_sorted();
// test_string()
// }

macro_rules! read_int(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        line.trim().parse::<usize>().unwrap()
    }}
);

macro_rules! read_line(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        String::from(line.trim())
    }}
);

fn main() {
    let s = read_line!();
    let n = read_int!();
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

    println!("{}", number_of_as);
}
