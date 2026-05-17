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
    type CreateProductResponse,
    type DeleteProductResponse,
    type PaginatedProductResolvedListResponse,
    type ProductResolvedResponse,
    type ProductResponse,
    type ProductUserInput,
    type UpdateProductResponse,
} from "@/components/modules/products/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type ProcessedJsonResponse,
    ProcessJsonResponse,
    type SelectOptionListResponse,
    type ProcessedBytesResponse,
    ProcessBytesResponse,
} from "@/lib/interface.ts";
import {
    isCreateProductResponse,
    isDeleteProductResponse,
    isPaginatedProductResolvedListResponse,
    isProductResolvedResponse,
    isProductResponse,
    isUpdateProductResponse,
} from "@/components/modules/products/lib/guards.ts";

export async function create(
    {
        id,
        name,
        description,
        unitOfMeasureId,
        newUnitOfMeasure,
        status,
    }: ProductUserInput,
    token: string | null,
): Promise<ProcessedJsonResponse<CreateProductResponse>> {
    return await fetch(`/api/products/create`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id,
            name,
            description,
            unit_of_measure_id: unitOfMeasureId,
            new_unit_of_measure: newUnitOfMeasure,
            status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isCreateProductResponse)) ??
            unexpectedFormError
        );
    });
}

export async function list(
    query: string | null,
    token: string | null,
): Promise<ProcessedJsonResponse<PaginatedProductResolvedListResponse>> {
    const uri =
        query === null ? `/api/products/list` : `/api/products/list?q=${query}`;
    return await fetch(uri, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isPaginatedProductResolvedListResponse,
            )) ?? unexpectedError
        );
    });
}

export async function select_list(
    list: string,
    token: string | null,
): Promise<ProcessedJsonResponse<SelectOptionListResponse>> {
    return await fetch(`/api/products/select_list?list=${list}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isSelectOptionListResponse)) ??
            unexpectedError
        );
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<ProductResolvedResponse>> {
    return await fetch(`/api/products/get_resolved?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isProductResolvedResponse)) ??
            unexpectedError
        );
    });
}

export async function update(
    {
        id,
        name,
        description,
        unitOfMeasureId,
        newUnitOfMeasure,
        status,
    }: ProductUserInput,
    token: string | null,
): Promise<ProcessedJsonResponse<UpdateProductResponse>> {
    return await fetch(`/api/products/update`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id,
            name,
            description,
            unit_of_measure_id: unitOfMeasureId,
            new_unit_of_measure: newUnitOfMeasure,
            status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isUpdateProductResponse)) ??
            unexpectedFormError
        );
    });
}

export async function get(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<ProductResponse>> {
    return await fetch(`/api/products/get?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isProductResponse)) ??
            unexpectedError
        );
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<DeleteProductResponse>> {
    return await fetch(`/api/products/delete?uuid=${uuid}`, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isDeleteProductResponse)) ??
            unexpectedError
        );
    });
}

export async function print(
    uuid: string,
    token: string | null,
): Promise<ProcessedBytesResponse> {
    return await fetch(`/api/products/print?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return ProcessBytesResponse(response);
    });
}
