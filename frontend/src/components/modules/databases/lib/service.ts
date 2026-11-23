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
    type ActiveDatabaseResponse,
    type CreateDatabase,
    type CreateDatabaseResponse,
    type DatabaseError,
    type DatabaseResponse,
    isActiveDatabaseResponse,
    isCreateDatabaseResponse,
    isDatabaseError,
    isDatabaseResponse,
    isPaginatedDatabaseListResponse,
    type PaginatedDatabaseListResponse,
} from "@/components/modules/databases/lib/interface.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    { name }: CreateDatabase,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateDatabaseResponse, DatabaseError>> {
    return apiRequest("/api/tenants/create", {
        method: "POST",
        token,
        signal,
        body: { name },
        guard: isCreateDatabaseResponse,
        formErrorGuard: isDatabaseError,
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedDatabaseListResponse>> {
    const uri =
        query === null ? "/api/tenants/list" : `/api/tenants/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedDatabaseListResponse,
    });
}

export async function activate(
    uuid: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ActiveDatabaseResponse>> {
    return apiRequest("/api/tenants/activate", {
        method: "POST",
        token,
        signal,
        body: { uuid },
        guard: isActiveDatabaseResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DatabaseResponse>> {
    return apiRequest(`/api/tenants/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isDatabaseResponse,
    });
}

export async function deleteDatabase(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ActiveDatabaseResponse>> {
    return apiRequest("/api/tenants/delete", {
        method: "POST",
        token,
        signal,
        body: { uuid },
        guard: isActiveDatabaseResponse,
    });
}
