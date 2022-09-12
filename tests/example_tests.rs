/*
 * File: example_tests.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * parameters passed into assert_eq!() or assert_ne!() have to implement partial equal and debug traits
 */

use testing::struct_guess::Guess;
use testing::struct_rectangle::Rectangle;
use testing::tested::*;

use crate::common::shared_test;

mod common;

#[test]
fn it_works() {
    // Asserting that terms are equal, else panic!()
    assert_eq!(2 + 2, 4);
}

// Should panic!()
#[test]
#[should_panic]
fn it_fails() {
    assert_eq!(2 + 1, 4);
}

#[test]
fn larger_can_hold_smaller_and_smaller_can_not_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };
    // Asserting that expression evaluates to true, else panic!()
    assert!(larger.can_hold(&smaller) && !smaller.can_hold(&larger));
}

#[test]
fn it_adds_two() {
    let a = 2;
    assert_eq!(a + 2, add_two(&a));
}

#[test]
fn it_does_not_add_three() {
    let a = 2;
    // Asserting that terms are not equal, else panic!()
    assert_ne!(add_three(&a), add_two(&a));
}

// Custom failed message
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was '{}'",
        result
    );
}

// Should panic!() with expected message -> doesn't compare the whole string, just checks if it contains
#[test]
#[should_panic(expected = "Value must be between 1 and 100")]
fn greater_than_100() {
    Guess::new(200);
}

// Tests that return a Result type, If Err() is reached test fails. ? can help you specify a message
// for specific failures
#[test]
fn return_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal 4"))
    }
}

// cargo test run_only_this_test to only run a specific test
#[test]
fn run_only_this_test() {
    assert_eq!(102, add_two(&100));
}

// cargo test run_ to run this test too
#[test]
fn run_this_test_too() {
    assert_eq!(102, add_two(&100));
}

// cargo test -- --ignored to run this test
#[test]
#[ignore]
fn ignore_this_test() {
    assert_eq!(102, add_two(&100));
}

#[test]
fn run_shared_test() {
    shared_test(&2).expect("ERROR: Function did not add 2");
}

// cargo test example_tests::tests_collection:: to run all tests inside this
#[cfg(test)]
mod tests_collection {
    use testing::tested::add_two;

    #[test]
    fn run_this_test_0() {
        assert_eq!(102, add_two(&100));
    }

    #[test]
    fn run_this_test_1() {
        assert_eq!(102, add_two(&100));
    }

    #[test]
    fn run_this_test_2() {
        assert_eq!(102, add_two(&100));
    }

    #[test]
    fn run_this_test_3() {
        assert_eq!(102, add_two(&100));
    }
}
