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
  type FormError,
  isCommonResponse,
  isFormError,
  isSimpleError,
  isSimpleMessageData,
  type SimpleMessageData,
  type SimpleError,
} from "@/lib/interfaces/common.ts";
import type {
  Claims,
  ClaimsResponse,
  LoginData,
  LoginResponse,
  LoginUser,
  RegisterResponse,
  VerifyEmailResponse
} from "@/components/modules/auth/lib/interface.ts";

export function isClaims(data: unknown): data is Claims {
  return (
    typeof data === "object" &&
    data !== null &&
    "sub" in data &&
    typeof data.sub === "string" &&
    "exp" in data &&
    typeof data.exp === "number" &&
    "iat" in data &&
    typeof data.iat === "number" &&
    "nbf" in data &&
    typeof data.nbf === "number" &&
    "iss" in data &&
    typeof data.iss === "string" &&
    "aud" in data &&
    typeof data.aud === "string" &&
    "jti" in data &&
    typeof data.jti === "string" &&
    "active_tenant" in data &&
    (data.active_tenant === null || typeof data.active_tenant === "string")
  );
}

export function isClaimsResponse(data: unknown): data is ClaimsResponse {
  return isCommonResponse(
    data,
    isClaims,
    isSimpleError,
  )
}

export function isLoginUser(data: unknown): data is LoginUser {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "email" in data &&
    typeof data.email === "string" &&
    "first_name" in data &&
    (data.first_name === null || typeof data.first_name === "string") &&
    "last_name" in data &&
    (data.last_name === null || typeof data.last_name === "string") &&
    "status" in data &&
    typeof data.status === "string" &&
    "profile_picture_url" in data &&
    (data.profile_picture_url === null || typeof data.profile_picture_url === "string")
  );
}

export function isLoginData(data: unknown): data is LoginData {
  return (
    typeof data === "object" &&
    data !== null &&
    "claims" in data &&
    isClaims(data.claims) &&
    "user" in data &&
    isLoginUser(data.user) &&
    "token" in data &&
    typeof data.token === "string"
  );
}

export function isLoginResponse(data: unknown): data is LoginResponse {
  return isCommonResponse<LoginData, FormError>(
    data,
    isLoginData,
    isFormError,
  );
}

export function isRegisterResponse(data: unknown): data is RegisterResponse {
  return isCommonResponse<SimpleMessageData, FormError>(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isVerifyEmailResponse(data: unknown): data is VerifyEmailResponse {
  return isCommonResponse<SimpleMessageData, SimpleError>(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}
