#![allow(dead_code, unused_imports, unused_must_use)]

#[macro_use]
extern crate maplit;

use crate::closure_with_result_errors::{closure_with_result, closure_with_result_other};
use crate::filter::test_filter_map;
use crate::group_by::group_by_second_example;
use crate::merge_join_by::merge_join_by_example;
use crate::option::option;
use crate::reference::reference_test;
use crate::sorted_enum::print_sorted_enum;
use crate::threads::{
    thread_that_dont_wait_and_finish_after_main, thread_that_waits_and_finish_before_main,
};

mod closure_with_result_errors;
mod filter;
mod group_by;
mod merge_join_by;
mod option;
mod reference;
mod serde_deserialization;
mod sorted_enum;
mod threads;

fn main() {
    // reference_test();
    // group_by_second_example()
    // closure_with_result_other();
    // option();
    // thread_that_dont_wait_and_finish_after_main();
    // thread_that_waits_and_finish_before_main();
    // print_sorted_enum();
    // test_filter_map();
    merge_join_by_example();
}
