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

export interface ApiErrorBody<E> {
    error: {
        message: string;
        fields?: E;
    };
}

export type ApiResult<T, E = never> =
    | { kind: "success"; status: number; data: T }
    | { kind: "http_error"; status: number; body?: ApiErrorBody<E> }
    | { kind: "rate_limited"; status: 429; retryAfterSeconds?: number }
    | { kind: "invalid_response"; status: number } // body didn't match the guard
    | { kind: "timeout" }
    | { kind: "network_error"; offline: boolean }
    | { kind: "aborted" }; // aborted by the caller (unmount, navigation)
