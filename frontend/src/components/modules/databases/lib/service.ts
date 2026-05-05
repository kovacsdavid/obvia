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
    type ActiveDatabaseResponse,
    type CreateDatabase,
    type CreateDatabaseResponse,
    type DatabaseResponse,
    isActiveDatabaseResponse,
    isCreateDatabaseResponse,
    isDatabaseResponse,
    isPaginatedDatabaseListResponse,
    type PaginatedDatabaseListResponse,
} from "@/components/modules/databases/lib/interface.ts";
import {
    type ProcessedJsonResponse,
    ProcessJsonResponse,
} from "@/lib/interface.ts";

export async function create(
    { name }: CreateDatabase,
    token: string | null,
): Promise<ProcessedJsonResponse<CreateDatabaseResponse>> {
    return await fetch(`/api/tenants/create`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({
            name,
        }),
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isCreateDatabaseResponse)) ??
            unexpectedFormError
        );
    });
}

export async function list(
    query: string | null,
    token: string | null,
): Promise<ProcessedJsonResponse<PaginatedDatabaseListResponse>> {
    const uri =
        query === null ? `/api/tenants/list` : `/api/tenants/list?q=${query}`;
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
                isPaginatedDatabaseListResponse,
            )) ?? unexpectedFormError
        );
    });
}

export async function activate(
    uuid: string | null,
    token: string | null,
): Promise<ProcessedJsonResponse<ActiveDatabaseResponse>> {
    return await fetch(`/api/tenants/activate`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({
            uuid,
        }),
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isActiveDatabaseResponse)) ??
            unexpectedError
        );
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<DatabaseResponse>> {
    return await fetch(`/api/tenants/get?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isDatabaseResponse)) ??
            unexpectedError
        );
    });
}

export async function deleteDatabase(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<ActiveDatabaseResponse>> {
    return await fetch(`/api/tenants/delete`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({
            uuid,
        }),
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isActiveDatabaseResponse)) ??
            unexpectedError
        );
    });
}
