/*
 * File: unit_tests.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * Unit tests are tests within the same file as the code
 */

#[allow(dead_code)]
fn add_one(a: &i32) -> i32 {
    a + 1
}

// Convention: Add a mod tests{}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_plus_one() {
        assert_eq!(2, add_one(&1));
    }
}
