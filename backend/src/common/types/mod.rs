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

pub(crate) mod currency_code;
pub(crate) mod ddl_parameter;
pub(crate) mod email;
pub(crate) mod first_name;
pub(crate) mod last_name;
pub(crate) mod order;
pub(crate) mod password;
pub(crate) mod value_object;

pub(crate) use ddl_parameter::DdlParameter;
pub(crate) use email::Email;
pub(crate) use first_name::FirstName;
pub(crate) use last_name::LastName;
pub(crate) use password::Password;
