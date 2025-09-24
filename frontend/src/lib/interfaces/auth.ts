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
  type CommonResponse,
  type FormError,
  isCommonResponse,
  isFormError, isSimpleMessageResponse,
  type SimpleMessageResponse
} from "@/lib/interfaces/common.ts";


export interface LoginRequest {
  email: string,
  password: string,
}

export interface LoginData {
  user: LoginUser,
  token: string,
}

export interface LoginUser {
  id: string;
  email: string;
  first_name: string | null;
  last_name: string | null;
  status: string;
  profile_picture_url: string | null;
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
    "user" in data &&
    isLoginUser(data.user) &&
    "token" in data &&
    typeof data.token === "string"
  );
}

export function isLoginResponse(data: unknown): data is CommonResponse<LoginData, FormError> {
  return isCommonResponse<LoginData, FormError>(
    data,
    isLoginData,
    isFormError,
  );
}

export interface RegisterRequest {
  firstName: string,
  lastName: string,
  email: string,
  password: string,
  passwordConfirm: string,
}

export function isRegisterResponse(data: unknown): data is CommonResponse<SimpleMessageResponse, FormError> {
  return isCommonResponse<SimpleMessageResponse, FormError>(
    data,
    isSimpleMessageResponse,
    isFormError
  )
}

