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

import {globalRequestTimeout, unexpectedError, unexpectedFormError} from "@/services/utils/consts.ts";
import {
  type CreateCustomer,
  type CreateCustomerResponse,
  isCreateCustomerResponse,
  isPaginatedCustomerResolvedListResponse,
  type PaginatedCustomerResolvedListResponse
} from "@/components/customers/interface.ts";
import {type ProcessedResponse, ProcessResponse} from "@/lib/interfaces/common.ts";

export async function create({
                               name,
                               contactName,
                               email,
                               phoneNumber,
                               status,
                               customerType,
                             }: CreateCustomer, token: string | null): Promise<ProcessedResponse<CreateCustomerResponse>> {
  return await fetch(`/api/customers/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      name,
      contact_name: contactName,
      email,
      phone_number: phoneNumber,
      status: typeof status === "undefined" ? null : status,
      customer_type: typeof customerType === "undefined" ? null : customerType,
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateCustomerResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedCustomerResolvedListResponse>> {
  const uri = query === null ? `/api/customers/list` : `/api/customers/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isPaginatedCustomerResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<PaginatedCustomerResolvedListResponse>> {
  return await fetch(`/api/customers/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isPaginatedCustomerResolvedListResponse
    ) ?? unexpectedError;
  });
}