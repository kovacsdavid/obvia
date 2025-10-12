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
pub(crate) mod legal_text;
pub(crate) mod order_by;
pub(crate) mod rate;
pub(crate) mod reporting_code;
pub(crate) mod status;
pub(crate) mod tax_category;

pub(crate) use description::Description as TaxDescription;
pub(crate) use legal_text::LegalText as TaxLegalText;
pub(crate) use order_by::OrderBy as TaxOrderBy;
pub(crate) use rate::Rate as TaxRate;
pub(crate) use reporting_code::ReportingCode as TaxReportingCode;
pub(crate) use status::Status as TaxStatus;
pub(crate) use tax_category::TaxCategory;
