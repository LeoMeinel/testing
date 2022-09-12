/*
 * File: struct_guess.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Value must be between 1 and 100");
        }
        Guess { value }
    }
}
