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
  CreateServiceResponse,
  DeleteServiceResponse,
  PaginatedServiceResolvedListResponse,
  Service,
  ServiceResolved,
  ServiceResolvedList,
  ServiceResolvedResponse,
  ServiceResponse,
  UpdateServiceResponse
} from "@/components/services/interface.ts";

export function isCreateServiceResponse(data: unknown): data is CreateServiceResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isUpdateServiceResponse(data: unknown): data is UpdateServiceResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteServiceResponse(data: unknown): data is DeleteServiceResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}

export function isServiceResolved(data: unknown): data is ServiceResolved {
  return (
    typeof data === 'object' &&
    data !== null &&
    'id' in data &&
    typeof data.id === 'string' &&
    'name' in data &&
    typeof data.name === 'string' &&
    'description' in data &&
    (data.description === null || typeof data.description === 'string') &&
    'default_price' in data &&
    (data.default_price === null || typeof data.default_price === 'string') &&
    'default_tax_id' in data &&
    (data.default_tax_id === null || typeof data.default_tax_id === 'string') &&
    'default_tax' in data &&
    (data.default_tax === null || typeof data.default_tax === 'string') &&
    'currency_id' in data &&
    (data.currency_id === null || typeof data.currency_id === 'string') &&
    'currency' in data &&
    (data.currency === null || typeof data.currency === 'string') &&
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

export function isServiceResolvedResponse(data: unknown): data is ServiceResolvedResponse {
  return isCommonResponse(
    data,
    isServiceResolved,
    isSimpleError
  )
}

export function isServiceResolvedList(data: unknown): data is ServiceResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isServiceResolved(item))
  );
}

export function isPaginatedServiceResolvedListResponse(
  data: unknown
): data is PaginatedServiceResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isServiceResolvedList
  );
}

export function isService(data: unknown): data is Service {
  return (
    typeof data === 'object' &&
    data !== null &&
    'id' in data &&
    typeof data.id === 'string' &&
    'name' in data &&
    typeof data.name === 'string' &&
    'description' in data &&
    (data.description === null || typeof data.description === 'string') &&
    'default_price' in data &&
    (data.default_price === null || typeof data.default_price === 'string') &&
    'default_tax_id' in data &&
    (data.default_tax_id === null || typeof data.default_tax_id === 'string') &&
    'currency_id' in data &&
    (data.currency_id === null || typeof data.currency_id === 'string') &&
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

export function isServiceResponse(data: unknown): data is ServiceResponse {
  return isCommonResponse(
    data,
    isService,
    isSimpleError
  )
}