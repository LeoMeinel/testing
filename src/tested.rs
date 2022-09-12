/*
 * File: tested.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[allow(dead_code)]
pub fn tested() {}

#[allow(dead_code)]
pub fn add_two(a: &i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
pub fn add_three(a: &i32) -> i32 {
    a + 3
}

#[allow(dead_code)]
pub fn greeting(name: &str) -> String {
    println!("This will only be shown with cargo test -- --show-output");
    format!("Hello {}", name)
}
