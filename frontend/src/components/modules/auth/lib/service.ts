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
  type ClaimsResponse,
  type ForgottenPasswordRequest,
  type ForgottenPasswordResponse,
  type LoginRequest,
  type NewPasswordRequest,
  type NewPasswordResponse,
  type RegisterRequest,
  type RegisterResponse,
  type VerifyEmailResponse,
} from "@/components/modules/auth/lib/interface.ts";
import {
  type ProcessedResponse,
  ProcessResponse,
} from "@/lib/interfaces/common.ts";
import {
  isClaimsResponse,
  isForgottenPasswordResponse,
  isNewPasswordResponse,
  isRegisterResponse,
  isVerifyEmailResponse,
} from "@/components/modules/auth/lib/guards.ts";

export async function login({
  email,
  password,
}: LoginRequest): Promise<Response> {
  return await fetch(`/api/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email, password }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}

export async function register({
  firstName,
  lastName,
  email,
  password,
  passwordConfirm,
}: RegisterRequest): Promise<ProcessedResponse<RegisterResponse>> {
  return await fetch(`/api/auth/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      first_name: firstName,
      last_name: lastName,
      email,
      password,
      password_confirm: passwordConfirm,
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isRegisterResponse)) ??
      unexpectedFormError
    );
  });
}

export async function get_claims(
  token: string | null,
): Promise<ProcessedResponse<ClaimsResponse>> {
  return await fetch(`/api/auth/get_claims`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isClaimsResponse)) ?? unexpectedError
    );
  });
}

export async function verfiy_email(
  id: string,
): Promise<ProcessedResponse<VerifyEmailResponse>> {
  return await fetch(`/api/auth/verify_email?id=${id}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isVerifyEmailResponse)) ??
      unexpectedError
    );
  });
}

export async function forgottenPassword(
  forgottenPasswordRequest: ForgottenPasswordRequest,
): Promise<ProcessedResponse<ForgottenPasswordResponse>> {
  return await fetch(`/api/auth/forgotten_password`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email: forgottenPasswordRequest.email,
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isForgottenPasswordResponse)) ??
      unexpectedError
    );
  });
}

export async function newPassword(
  newPasswordRequest: NewPasswordRequest,
): Promise<ProcessedResponse<NewPasswordResponse>> {
  return await fetch(`/api/auth/new_password`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      token: newPasswordRequest.token,
      password: newPasswordRequest.password,
      password_confirm: newPasswordRequest.password_confirm,
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isNewPasswordResponse)) ??
      unexpectedError
    );
  });
}
