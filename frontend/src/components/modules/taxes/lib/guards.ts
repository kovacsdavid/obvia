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
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleError,
  isSimpleMessageData,
} from "@/lib/interfaces/common.ts";
import type {
  CreateTaxResponse,
  DeleteTaxResponse,
  PaginatedTaxResolvedListResponse,
  Tax,
  TaxResolved,
  TaxResolvedList,
  TaxResolvedResponse,
  TaxResponse,
  UpdateTaxResponse
} from "@/components/modules/taxes/lib/interface.ts";

export function isCreateTaxResponse(data: unknown): data is CreateTaxResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isUpdateTaxResponse(data: unknown): data is UpdateTaxResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteTaxResponse(data: unknown): data is DeleteTaxResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}

export function isTaxResolved(data: unknown): data is TaxResolved {
  return (
    typeof data === 'object' &&
    data !== null &&
    'id' in data &&
    typeof data.id === 'string' &&
    'rate' in data &&
    (data.rate === null || typeof data.rate === 'string') &&
    'description' in data &&
    typeof data.description === 'string' &&
    'country_code' in data &&
    typeof data.country_code === 'string' &&
    'country' in data &&
    typeof data.country === 'string' &&
    'tax_category' in data &&
    typeof data.tax_category === 'string' &&
    'is_rate_applicable' in data &&
    typeof data.is_rate_applicable === 'boolean' &&
    'legal_text' in data &&
    (data.legal_text === null || typeof data.legal_text === 'string') &&
    'reporting_code' in data &&
    (data.reporting_code === null || typeof data.reporting_code === 'string') &&
    'is_default' in data &&
    typeof data.is_default === 'boolean' &&
    'status' in data &&
    typeof data.status === 'string' &&
    'created_by_id' in data &&
    typeof data.created_by_id === 'string' &&
    'created_by' in data &&
    typeof data.created_by === 'string' &&
    'created_at' in data &&
    typeof data.created_at === 'string' &&
    'updated_at' in data &&
    typeof data.updated_at === 'string' &&
    'deleted_at' in data &&
    (data.deleted_at === null || typeof data.deleted_at === 'string')
  );
}

export function isTaxResolvedResponse(data: unknown): data is TaxResolvedResponse {
  return isCommonResponse(
    data,
    isTaxResolved,
    isSimpleError
  )
}

export function isTaxResolvedList(data: unknown): data is TaxResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isTaxResolved(item))
  );
}

export function isPaginatedTaxResolvedListResponse(
  data: unknown
): data is PaginatedTaxResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isTaxResolvedList
  );
}

export function isTax(data: unknown): data is Tax {
  return (
    typeof data === 'object' &&
    data !== null &&
    'id' in data &&
    typeof data.id === 'string' &&
    'rate' in data &&
    (data.rate === null || typeof data.rate === 'string') &&
    'description' in data &&
    typeof data.description === 'string' &&
    'country_code' in data &&
    typeof data.country_code === 'string' &&
    'tax_category' in data &&
    typeof data.tax_category === 'string' &&
    'is_rate_applicable' in data &&
    typeof data.is_rate_applicable === 'boolean' &&
    'legal_text' in data &&
    (data.legal_text === null || typeof data.legal_text === 'string') &&
    'reporting_code' in data &&
    (data.reporting_code === null || typeof data.reporting_code === 'string') &&
    'is_default' in data &&
    typeof data.is_default === 'boolean' &&
    'status' in data &&
    typeof data.status === 'string' &&
    'created_by_id' in data &&
    typeof data.created_by_id === 'string' &&
    'created_at' in data &&
    typeof data.created_at === 'string' &&
    'updated_at' in data &&
    typeof data.updated_at === 'string' &&
    'deleted_at' in data &&
    (data.deleted_at === null || typeof data.deleted_at === 'string')
  );
}

export function isTaxResponse(data: unknown): data is TaxResponse {
  return isCommonResponse(
    data,
    isTax,
    isSimpleError
  )
}