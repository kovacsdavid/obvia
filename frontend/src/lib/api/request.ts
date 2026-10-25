/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import type { ApiErrorBody, ApiResult } from "@/lib/api/types";
import { globalRequestTimeout } from "@/services/utils/consts";

type Guard<T> = (data: unknown) => data is T;

// Default when no form guard is given: any `fields` invalidates the body
const noFields = (_: unknown): _ is never => false;

interface RequestOptions<T, E> {
    method?: "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
    token?: string | null;
    body?: unknown;
    guard: Guard<T>;
    formErrorGuard?: Guard<E>; // omit for requests without form validation
    signal?: AbortSignal; // caller-controlled cancellation
    timeoutMs?: number;
}

export function isApiErrorBody<E>(
    data: unknown,
    formErrorGuard: Guard<E>,
): data is ApiErrorBody<E> {
    if (typeof data !== "object" || data === null) return false;

    const err = (data as { error?: unknown }).error;
    if (typeof err !== "object" || err === null) return false;

    const { message, fields } = err as { message?: unknown; fields?: unknown };
    if (typeof message !== "string") return false;

    return fields === undefined || formErrorGuard(fields);
}

function parseRetryAfter(header: string | null): number | undefined {
    if (!header) return undefined;
    const seconds = Number(header);
    if (Number.isFinite(seconds)) return Math.max(0, seconds);
    const date = Date.parse(header);
    return Number.isNaN(date)
        ? undefined
        : Math.max(0, Math.ceil((date - Date.now()) / 1000));
}

function classifyTransportError(
    e: unknown,
    timeoutSignal: AbortSignal,
):
    | { kind: "timeout" }
    | { kind: "aborted" }
    | { kind: "network_error"; offline: boolean } {
    if (e instanceof DOMException) {
        if (e.name === "TimeoutError") return { kind: "timeout" };
        if (e.name === "AbortError") {
            // AbortSignal.any forwards the reason, so check which signal fired
            return timeoutSignal.aborted
                ? { kind: "timeout" }
                : { kind: "aborted" };
        }
    }
    // TypeError on DNS, CORS, connection refused, offline, or a body stream cut mid-download
    return { kind: "network_error", offline: !navigator.onLine };
}

const MALFORMED = Symbol("malformed");

// Read errors (abort/timeout/network) propagate; only a JSON parse failure returns MALFORMED.
async function processResponse(response: Response): Promise<unknown> {
    if (response.status === 204) return undefined;
    const type = response.headers.get("Content-Type") ?? "";
    if (type.includes("application/json")) {
        const text = await response.text(); // transport errors propagate
        if (text.length === 0) return undefined;
        try {
            return JSON.parse(text);
        } catch {
            return MALFORMED;
        }
    }
    if (type.includes("application/pdf")) {
        return await response.bytes(); // transport errors propagate
    }
    return undefined;
}

export async function apiRequest<T, E = never>(
    url: string,
    {
        method = "GET",
        token,
        body,
        guard,
        formErrorGuard = noFields,
        signal,
        timeoutMs = globalRequestTimeout,
    }: RequestOptions<T, E>,
): Promise<ApiResult<T, E>> {
    const timeoutSignal = AbortSignal.timeout(timeoutMs);
    const combined = signal
        ? AbortSignal.any([signal, timeoutSignal])
        : timeoutSignal;

    let response: Response;
    try {
        response = await fetch(url, {
            method,
            headers: {
                Accept: "application/json",
                ...(body !== undefined
                    ? { "Content-Type": "application/json" }
                    : {}),
                ...(token ? { Authorization: `Bearer ${token}` } : {}),
            },
            body: body !== undefined ? JSON.stringify(body) : undefined,
            signal: combined,
        });
    } catch (e) {
        return classifyTransportError(e, timeoutSignal);
    }

    // Check 429 before parsing, because the edge router's body is usually not our JSON
    if (response.status === 429) {
        return {
            kind: "rate_limited",
            status: 429,
            retryAfterSeconds: parseRetryAfter(
                response.headers.get("Retry-After"),
            ),
        };
    }

    let data: unknown;
    try {
        data = await processResponse(response);
    } catch (e) {
        return classifyTransportError(e, timeoutSignal);
    }

    if (response.ok) {
        return data !== MALFORMED && guard(data)
            ? { kind: "success", status: response.status, data }
            : { kind: "invalid_response", status: response.status };
    }

    return {
        kind: "http_error",
        status: response.status,
        body: isApiErrorBody(data, formErrorGuard) ? data : undefined,
    };
}
