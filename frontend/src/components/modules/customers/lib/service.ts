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
  globalRequestTimeout,
  unexpectedError,
  unexpectedFormError,
} from "@/services/utils/consts.ts";
import {
  type CreateCustomerResponse,
  type CustomerResolvedResponse,
  type CustomerResponse,
  type CustomerUserInput,
  type DeleteCustomerResponse,
  type PaginatedCustomerResolvedListResponse,
  type UpdateCustomerResponse,
} from "@/components/modules/customers/lib/interface.ts";
import {
  type ProcessedResponse,
  ProcessResponse,
} from "@/lib/interfaces/common.ts";
import {
  isCreateCustomerResponse,
  isCustomerResolvedResponse,
  isCustomerResponse,
  isDeleteCustomerResponse,
  isPaginatedCustomerResolvedListResponse,
  isUpdateCustomerResponse,
} from "@/components/modules/customers/lib/guards.ts";

export async function create(
  {
    id,
    name,
    contactName,
    email,
    phoneNumber,
    status,
    customerType,
  }: CustomerUserInput,
  token: string | null,
): Promise<ProcessedResponse<CreateCustomerResponse>> {
  return await fetch(`/api/customers/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      contact_name: contactName,
      email,
      phone_number: phoneNumber,
      status: typeof status === "undefined" ? null : status,
      customer_type: typeof customerType === "undefined" ? null : customerType,
    }),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isCreateCustomerResponse)) ??
      unexpectedFormError
    );
  });
}

export async function update(
  {
    id,
    name,
    contactName,
    email,
    phoneNumber,
    status,
    customerType,
  }: CustomerUserInput,
  token: string | null,
): Promise<ProcessedResponse<UpdateCustomerResponse>> {
  return await fetch(`/api/customers/update`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      contact_name: contactName,
      email,
      phone_number: phoneNumber,
      status: typeof status === "undefined" ? null : status,
      customer_type: typeof customerType === "undefined" ? null : customerType,
    }),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isUpdateCustomerResponse)) ??
      unexpectedFormError
    );
  });
}

export async function list(
  query: string | null,
  token: string | null,
): Promise<ProcessedResponse<PaginatedCustomerResolvedListResponse>> {
  const uri =
    query === null ? `/api/customers/list` : `/api/customers/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(
        response,
        isPaginatedCustomerResolvedListResponse,
      )) ?? unexpectedError
    );
  });
}

export async function get_resolved(
  uuid: string,
  token: string | null,
): Promise<ProcessedResponse<CustomerResolvedResponse>> {
  return await fetch(`/api/customers/get_resolved?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isCustomerResolvedResponse)) ??
      unexpectedError
    );
  });
}

export async function get(
  uuid: string,
  token: string | null,
): Promise<ProcessedResponse<CustomerResponse>> {
  return await fetch(`/api/customers/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isCustomerResponse)) ?? unexpectedError
    );
  });
}

export async function deleteItem(
  uuid: string,
  token: string | null,
): Promise<ProcessedResponse<DeleteCustomerResponse>> {
  return await fetch(`/api/customers/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isDeleteCustomerResponse)) ??
      unexpectedError
    );
  });
}
