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
} from "@/services/utils/consts.ts";
import type { CreateUser } from "@/components/modules/users/lib/interface.ts";
import {
  ProcessResponse,
  type ProcessedResponse,
} from "@/lib/interfaces/common";
import type {
    DisableOtpResponse,
  EnableOtpResponse,
  VerifyOtpResponse,
} from "../../auth/lib/interface";
import {
    isDisableOtpResponse,
  isEnableOtpResponse,
  isVerifyOtpResponse,
} from "../../auth/lib/guards";

export async function create(
  { email, lastName, firstName, phone, status }: CreateUser,
  token: string | null,
): Promise<Response> {
  return await fetch(`/api/users/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      email,
      last_name: lastName,
      first_name: firstName,
      phone,
      status,
    }),
  });
}

export async function list(
  query: string | null,
  token: string | null,
): Promise<Response> {
  const uri = query === null ? `/api/users/list` : `/api/users/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}

export async function enableOtp(
  token: string | null,
): Promise<ProcessedResponse<EnableOtpResponse>> {
  return await fetch(`/api/users/otp/enable`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isEnableOtpResponse)) ?? unexpectedError
    );
  });
}

export async function verifyOtp(
  otp: string,
  token: string | null,
): Promise<ProcessedResponse<VerifyOtpResponse>> {
  return await fetch(`/api/users/otp/verify`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify({
      otp,
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isVerifyOtpResponse)) ?? unexpectedError
    );
  });
}

export async function disableOtp(
  otp: string,
  token: string | null,
): Promise<ProcessedResponse<DisableOtpResponse>> {
  return await fetch(`/api/users/otp/disable`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify({
      otp,
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isDisableOtpResponse)) ?? unexpectedError
    );
  });
}
