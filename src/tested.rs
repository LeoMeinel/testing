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
