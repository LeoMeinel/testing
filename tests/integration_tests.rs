/*
 * File: integration_tests.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * Integration tests are tests within the tests directory
 * They are meant to only test the public API
 */

use testing::tested::add_two;

use crate::common::shared_test;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(&2));
}

#[test]
fn run_shared_test() {
    shared_test(&2).expect("ERROR: Function did not add 2");
}
