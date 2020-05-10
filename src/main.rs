#![allow(dead_code, unused_imports, unused_must_use)]

use crate::closure_with_result_errors::{closure_with_result, closure_with_result_other};
use crate::group_by::group_by_second_example;
use crate::option::option;
use crate::reference::reference_test;

mod closure_with_result_errors;
mod group_by;
mod option;
mod reference;

fn main() {
    // reference_test();
    // group_by_second_example()
    closure_with_result_other();
    // option();
}
