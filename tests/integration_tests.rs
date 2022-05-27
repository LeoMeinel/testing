/*
 * testing is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/testing/blob/main/LICENSE
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
