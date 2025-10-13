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
  type CreateServiceResponse,
  type DeleteServiceResponse,
  type PaginatedServiceResolvedListResponse,
  type ServiceResolvedResponse,
  type ServiceResponse,
  type ServiceUserInput,
  type UpdateServiceResponse
} from "@/components/services/interface.ts";
import {
  isSelectOptionListResponse,
  type ProcessedResponse,
  ProcessResponse,
  type SelectOptionListResponse
} from "@/lib/interfaces/common.ts";
import {
  isCreateServiceResponse,
  isDeleteServiceResponse,
  isPaginatedServiceResolvedListResponse,
  isServiceResolvedResponse,
  isServiceResponse,
  isUpdateServiceResponse
} from "@/components/services/guards.ts";

export async function create({
                               id,
                               name,
                               description,
                               defaultPrice,
                               defaultTaxId,
                               currencyCode,
                               status,
                             }: ServiceUserInput, token: string | null): Promise<ProcessedResponse<CreateServiceResponse>> {
  return await fetch(`/api/services/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      description,
      default_price: defaultPrice,
      default_tax_id: defaultTaxId,
      currency_code: currencyCode,
      status,
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateServiceResponse
    ) ?? unexpectedFormError;
  });
}

export async function update({
                               id,
                               name,
                               description,
                               defaultPrice,
                               defaultTaxId,
                               currencyCode,
                               status,
                             }: ServiceUserInput, token: string | null): Promise<ProcessedResponse<UpdateServiceResponse>> {
  return await fetch(`/api/services/update`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      description,
      default_price: defaultPrice,
      default_tax_id: defaultTaxId,
      currency_code: currencyCode,
      status,
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isUpdateServiceResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedServiceResolvedListResponse>> {
  const uri = query === null ? `/api/services/list` : `/api/services/list?q=${query}`;
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
      isPaginatedServiceResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<ServiceResolvedResponse>> {
  return await fetch(`/api/services/get_resolved?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isServiceResolvedResponse
    ) ?? unexpectedError;
  });
}

export async function get(uuid: string, token: string | null): Promise<ProcessedResponse<ServiceResponse>> {
  return await fetch(`/api/services/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isServiceResponse
    ) ?? unexpectedError;
  });
}

export async function deleteItem(uuid: string, token: string | null): Promise<ProcessedResponse<DeleteServiceResponse>> {
  return await fetch(`/api/services/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isDeleteServiceResponse
    ) ?? unexpectedError;
  });
}

export async function select_list(
  list: string,
  token: string | null): Promise<ProcessedResponse<SelectOptionListResponse>> {
  return await fetch(`/api/services/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isSelectOptionListResponse
    ) ?? unexpectedError;
  });
}
