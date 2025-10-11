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

pub(crate) mod description;
pub(crate) mod name;
pub(crate) mod order_by;
pub(crate) mod status;
pub(crate) mod default_price;

pub(crate) use default_price::DefaultPrice as ServiceDefaultPrice;
pub(crate) use description::Description as ServiceDescription;
pub(crate) use name::Name as ServiceName;
pub(crate) use order_by::OrderBy as ServiceOrderBy;
pub(crate) use status::Status as ServiceStatus;
