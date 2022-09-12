/*
 * File: mod.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * Shared code goes into mod.rs inside a separate folder to not create additional tests
 */

use testing::tested::add_two;

pub fn shared_test(a: &i32) -> Result<(), String> {
    if add_two(a) == a + 2 {
        Ok(())
    } else {
        Err(String::from("Function did not add two"))
    }
}
