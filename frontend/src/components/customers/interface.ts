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
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleMessageData,
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface CreateCustomer {
  name: string
  contactName: string
  email: string
  phoneNumber: string
  status: string | undefined
  customerType: string | undefined,
}

export type CreateCustomerResponse = CommonResponse<SimpleMessageData, FormError>;

export function isCreateCustomerResponse(data: unknown): data is CreateCustomerResponse {
  return isCommonResponse<SimpleMessageData, FormError>(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export interface CustomerResolved {
  id: string,
  name: string,
  contact_name: string | null,
  email: string,
  phone_number: string | null,
  status: string,
  created_by_id: string,
  created_by: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type CustomerResolvedList = CustomerResolved[];

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

export type CustomerResolvedResponse = CommonResponse<CustomerResolved, SimpleError>;

export function isCustomerResolvedResponse(data: unknown): data is CustomerResolvedResponse {
  return isCommonResponse(
    data,
    isCustomerResolved
  )
}

export function isCustomerResolvedList(data: unknown): data is CustomerResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isCustomerResolved(item))
  );
}

export type PaginatedCustomerResolvedListResponse = PaginatedDataResponse<CustomerResolvedList, SimpleError>

export function isPaginatedCustomerResolvedListResponse(
  data: unknown
): data is PaginatedCustomerResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isCustomerResolvedList
  );
}
