/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use regex::Regex;

pub fn extract_human_error(message: &str) -> String {
    const PREFIX: &str = "Failed to deserialize the JSON body into the target type: ";
    let msg = message.strip_prefix(PREFIX).unwrap_or(message).trim();

    let re_suffix = Regex::new(r"\s*at line \d+ column \d+\s*$").unwrap();
    let msg = re_suffix.replace(msg, "").trim().to_string();

    let re_field = Regex::new(r"^\w+:\s*").unwrap();
    let msg = re_field.replace(&msg, "").trim().to_string();

    msg.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_prefix_suffix_and_field() {
        let full = "Failed to deserialize the JSON body into the target type: last_name: Hibás vezetéknév formátum: '!!' at line 1 column 16";
        assert_eq!(extract_human_error(full), "Hibás vezetéknév formátum: '!!'");
    }

    #[test]
    fn it_removes_suffix_and_field_with_other_numbers() {
        let full = "Failed to deserialize the JSON body into the target type: first_name: Hibás keresztnév formátum: 'Béla' at line 7 column 8";
        assert_eq!(
            extract_human_error(full),
            "Hibás keresztnév formátum: 'Béla'"
        );
    }

    #[test]
    fn it_removes_prefix_and_field_if_no_suffix() {
        let full =
            "Failed to deserialize the JSON body into the target type: password: Jelszó túl rövid";
        assert_eq!(extract_human_error(full), "Jelszó túl rövid");
    }

    #[test]
    fn it_removes_only_field_if_no_prefix_or_suffix() {
        let msg = "email: Hibás email formátum";
        assert_eq!(extract_human_error(msg), "Hibás email formátum");
    }

    #[test]
    fn it_returns_unchanged_if_no_prefix_field_or_suffix() {
        let msg = "Hibás adat";
        assert_eq!(extract_human_error(msg), "Hibás adat");
    }
}
