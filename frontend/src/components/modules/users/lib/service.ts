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
    CreateUser,
    OtpFields,
} from "@/components/modules/users/lib/interface.ts";
import type { EnableOtpResponse } from "../../auth/lib/interface";
import { isEnableOtpResponse } from "../../auth/lib/guards";
import {
    isOtpFields,
    isOtpMessageResponse,
    isUnimplementedResponse,
} from "./guards";
import type { SimpleMessageData } from "@/lib/interface";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    { email, lastName, firstName, phone, status }: CreateUser,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<never>> {
    return apiRequest(`/api/users/create`, {
        method: "POST",
        token,
        signal,
        guard: isUnimplementedResponse,
        body: {
            email,
            last_name: lastName,
            first_name: firstName,
            phone,
            status,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<never>> {
    const uri =
        query === null ? `/api/users/list` : `/api/users/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isUnimplementedResponse,
    });
}

export async function enableOtp(
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<EnableOtpResponse>> {
    return apiRequest(`/api/users/otp/enable`, {
        method: "GET",
        token,
        signal,
        guard: isEnableOtpResponse,
    });
}

export async function verifyOtp(
    otp: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<{ data: SimpleMessageData }, OtpFields>> {
    return apiRequest(`/api/users/otp/verify`, {
        method: "POST",
        token,
        signal,
        body: { otp },
        guard: isOtpMessageResponse,
        formErrorGuard: isOtpFields,
    });
}

export async function disableOtp(
    otp: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<{ data: SimpleMessageData }, OtpFields>> {
    return apiRequest(`/api/users/otp/disable`, {
        method: "POST",
        token,
        signal,
        body: { otp },
        guard: isOtpMessageResponse,
        formErrorGuard: isOtpFields,
    });
}
