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

// TODO: this possibly can be simplified!

#[macro_export]
macro_rules! validate_optional_string {
    ($value:expr, $error_field:expr) => {
        match ValueObject::new($value).inspect_err(|e| $error_field = Some(e.to_string())) {
            Ok(val) => {
                if !val.extract().get_value().trim().is_empty() {
                    Some(val)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    };
}
