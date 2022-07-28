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
