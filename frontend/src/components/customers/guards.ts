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
  CreateCustomerResponse,
  Customer,
  CustomerResolved,
  CustomerResolvedList,
  CustomerResolvedResponse, CustomerResponse, DeleteCustomerResponse,
  PaginatedCustomerResolvedListResponse,
  UpdateCustomerResponse
} from "@/components/customers/interface.ts";

export function isCreateCustomerResponse(data: unknown): data is CreateCustomerResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isUpdateCustomerResponse(data: unknown): data is UpdateCustomerResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteCustomerResponse(data: unknown): data is DeleteCustomerResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}

export function isCustomerResolved(data: unknown): data is CustomerResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "contact_name" in data &&
    (data.contact_name === null || typeof data.contact_name === "string") &&
    "email" in data &&
    typeof data.email === "string" &&
    "phone_number" in data &&
    (data.phone_number === null || typeof data.phone_number === "string") &&
    "status" in data &&
    typeof data.status === "string" &&
    "customer_type" in data &&
    typeof data.customer_type === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isCustomerResolvedResponse(data: unknown): data is CustomerResolvedResponse {
  return isCommonResponse(
    data,
    isCustomerResolved,
    isSimpleError
  )
}

export function isCustomerResolvedList(data: unknown): data is CustomerResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isCustomerResolved(item))
  );
}

export function isPaginatedCustomerResolvedListResponse(
  data: unknown
): data is PaginatedCustomerResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isCustomerResolvedList
  );
}

export function isCustomer(data: unknown): data is Customer {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "contact_name" in data &&
    (data.contact_name === null || typeof data.contact_name === "string") &&
    "email" in data &&
    typeof data.email === "string" &&
    "phone_number" in data &&
    (data.phone_number === null || typeof data.phone_number === "string") &&
    "status" in data &&
    typeof data.status === "string" &&
    "customer_type" in data &&
    typeof data.customer_type === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isCustomerResponse(data: unknown): data is CustomerResponse {
  return isCommonResponse(
    data,
    isCustomer,
    isSimpleError
  )
}