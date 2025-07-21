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

export interface OrganizationalUnitRequest {
    name: string;
    dbHost: string;
    dbPort: number;
    dbName: string;
    dbUser: string;
    dbPassword: string;
}

export interface OrganizationalUnitResponse {
    success: boolean,
    data?: {
        message: string,
    },
    error?: {
        reference: string | null,
        global: string | null,
        fields: Record<string, string | null>,
    }
}

export function isOrganizationalUnitResponse(data: unknown): data is OrganizationalUnitResponse {
    return (
        typeof data === "object" &&
        data !== null &&
        "success" in data &&
        typeof data.success === "boolean" &&
        (
            !("data" in data) ||
            (typeof data.data === "object" &&
                data.data !== null &&
                "message" in data.data &&
                typeof data.data.message === "string")
        ) &&
        (
            !("error" in data) ||
            (typeof data.error === "object" &&
                data.error !== null &&
                "reference" in data.error &&
                (data.error.reference === null || typeof data.error.reference === "string") &&
                "global" in data.error &&
                (data.error.global === null || typeof data.error.global === "string") &&
                "fields" in data.error &&
                typeof data.error.fields === "object")
        )
    );
}

export async function create({
    name,
    dbHost,
    dbPort,
    dbName,
    dbUser,
    dbPassword

                             }: OrganizationalUnitRequest, token: string | null): Promise<OrganizationalUnitResponse> {
    const response = await fetch(`/api/organizational_units/create`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { "Authorization": `Bearer ${token}` } : {})

        },
        body: JSON.stringify({
            name,
            dbHost,
            dbPort,
            dbName,
            dbUser,
            dbPassword
        })
    });
    let responseJson;
    try {
        responseJson = await response.json();
    } catch {
        throw new Error("Server responded with invalid JSON format");
    }
    if (!isOrganizationalUnitResponse(responseJson)) {
        throw new Error("Server responded with invalid data");
    }
    return responseJson;
}