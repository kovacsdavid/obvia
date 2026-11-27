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

import type {
    CreateProductResponse,
    DeleteProductResponse,
    PaginatedProductResolvedListResponse,
    ProductError,
    ProductResolvedResponse,
    ProductResponse,
    ProductUserInput,
    UpdateProductResponse,
} from "./interface";
import {
    isCreateProductResponse,
    isDeleteProductResponse,
    isPaginatedProductResolvedListResponse,
    isProductError,
    isProductResolvedResponse,
    isProductResponse,
    isUpdateProductResponse,
} from "./guards";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    input: ProductUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateProductResponse, ProductError>> {
    return apiRequest("/api/products/create", {
        method: "POST",
        token,
        signal,
        guard: isCreateProductResponse,
        formErrorGuard: isProductError,
        body: input,
    });
}

export async function update(
    input: ProductUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateProductResponse, ProductError>> {
    return apiRequest("/api/products/update", {
        method: "PUT",
        token,
        signal,
        guard: isUpdateProductResponse,
        formErrorGuard: isProductError,
        body: input,
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedProductResolvedListResponse>> {
    const uri =
        query === null ? "/api/products/list" : `/api/products/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedProductResolvedListResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/products/select_list?list=${list}`, {
        method: "GET",
        token,
        signal,
        guard: isSelectOptionListResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ProductResolvedResponse>> {
    return apiRequest(`/api/products/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isProductResolvedResponse,
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ProductResponse>> {
    return apiRequest(`/api/products/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isProductResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteProductResponse>> {
    return apiRequest(`/api/products/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteProductResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/products/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
