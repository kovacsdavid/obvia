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

import {
  type CommonResponse,
  type FormError,
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface TaxUserInput {
  id: string | null;
  rate: string | null;
  description: string;
  countryCode: string;
  taxCategory: string;
  isRateApplicable: boolean;
  legalText: string | null;
  reportingCode: string | null;
  isDefault: boolean;
  status: string;
}

export interface Tax {
  id: string;
  rate: string | null;
  description: string;
  country_code: string;
  tax_category: string;
  is_rate_applicable: boolean;
  legal_text: string | null;
  reporting_code: string | null;
  is_default: boolean;
  status: string;
  created_by_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface TaxResolved {
  id: string;
  rate: string | null;
  description: string;
  country_code: string;
  country: string;
  tax_category: string;
  is_rate_applicable: boolean;
  legal_text: string | null;
  reporting_code: string | null;
  is_default: boolean;
  status: string;
  created_by_id: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export type CreateTaxResponse = CommonResponse<SimpleMessageData, FormError>;
export type UpdateTaxResponse = CommonResponse<SimpleMessageData, FormError>;
export type DeleteTaxResponse = CommonResponse<SimpleMessageData, SimpleError>;
export type TaxResolvedList = TaxResolved[];
export type TaxResponse = CommonResponse<Tax, SimpleError>;
export type TaxResolvedResponse = CommonResponse<TaxResolved, SimpleError>;
export type PaginatedTaxResolvedListResponse = PaginatedDataResponse<TaxResolvedList, SimpleError>
