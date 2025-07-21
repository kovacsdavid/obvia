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

export interface LoginRequest {
  email: string,
  password: string,
}

export interface LoginResponse {
  success: boolean,
  data?: {
    user: {
      id: string;
      email: string;
      first_name: string | null;
      last_name: string | null;
      status: string;
      profile_picture_url: string | null;
    },
    token: string,
  },
  error?: {
    reference: string | null,
    global: string | null,
    fields: Record<string, string | null>,
  }
}

export function isLoginResponse(data: unknown): data is LoginResponse {
  return (
    typeof data === "object" &&
    data !== null &&
    "success" in data &&
    typeof data.success === "boolean" &&
    (
      !("data" in data) ||
      (typeof data.data === "object" &&
        data.data !== null &&
        "user" in data.data &&
        typeof data.data.user === "object" &&
        data.data.user !== null &&
        "id" in data.data.user &&
        typeof data.data.user.id === "string" &&
        "email" in data.data.user &&
        typeof data.data.user.email === "string" &&
        "token" in data.data &&
        typeof data.data.token === "string")
    ) &&
    (
      !("error" in data) ||
      (typeof data.error === "object" &&
        data.error !== null &&
        "global" in data.error &&
        (data.error.global === null || typeof data.error.global === "string") &&
        "fields" in data.error &&
        typeof data.error.fields === "object")
    )
  );
}

export async function login({ email, password }: LoginRequest): Promise<LoginResponse> {
  const response = await fetch(`/api/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email, password }),
    signal: AbortSignal.timeout(10000),
  });
  let responseJson;
  try {
    responseJson = await response.json();
  } catch {
    throw new Error("Server responded with invalid JSON format");
  }
  if (!isLoginResponse(responseJson)) {
    throw new Error("Server responded with invalid data");
  }
  return responseJson;
}

export interface RegisterRequest {
  firstName: string,
  lastName: string,
  email: string,
  password: string,
  passwordConfirm: string,
}

export interface RegisterResponse {
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

export function isRegisterResponse(data: unknown): data is RegisterResponse {
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

export async function register({
                                 firstName,
                                 lastName,
                                 email,
                                 password,
                                 passwordConfirm
                               }: RegisterRequest): Promise<RegisterResponse> {
  const response = await fetch(`/api/auth/register`, {
    method: "POST",
    headers: {"Content-Type": "application/json"},
    body: JSON.stringify({
      first_name: firstName,
      last_name: lastName,
      email,
      password,
      password_confirm: passwordConfirm
    }),
    signal: AbortSignal.timeout(10000)
  });
  let responseJson;
  try {
    responseJson = await response.json();
  } catch {
    throw new Error("Server responded with invalid JSON format");
  }
  if (!isRegisterResponse(responseJson)) {
    throw new Error("Server responded with invalid data");
  }
  return responseJson;
}
