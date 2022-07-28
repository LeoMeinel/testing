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
 * along with this program. If not, see https://github.com/LeoMeinel/testing/blob/main/LICENSE
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
