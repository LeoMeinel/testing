/*
 * File: lib.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * For integration tests you need a lib.rs (Library crate) because you can not integration test on
 * only main.rs (Binary crate)
 */

pub mod struct_guess;
pub mod struct_rectangle;
pub mod tested;
mod unit_tests;
