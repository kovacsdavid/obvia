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

export interface SimpeError {
  global: string | null | undefined
}

export interface FormError {
  global: string | null | undefined
  fields: Record<string, string> | null | undefined
}

export interface CommonResponse<T, E> {
  success: boolean,
  data?: T,
  error?: E
}

export interface SimpleMessageData {
  message: string
}

export function isSimpleError(data: unknown): data is SimpeError {
  return (
    typeof data === "object" &&
    data !== null &&
    "global" in data &&
    (data.global === null || data.global === undefined || typeof data.global === "string")
  );
}

export function isFormError(data: unknown): data is FormError {
  return (typeof data === "object" &&
    data !== null &&
    "global" in data &&
    (data.global === null || data.global === undefined || typeof data.global === "string") &&
    "fields" in data && (data.fields === null ||
      data.fields === undefined || (typeof data.fields === "object" && true && Object.values(data.fields).every(value => typeof value === "string"))));
}

export function isCommonResponse<T, E>(
  data: unknown,
  dataGuard?: (value: unknown) => value is T,
  errorGuard?: (value: unknown) => value is E
): data is CommonResponse<T, E> {
  if (
    typeof data !== "object" ||
    data === null ||
    !("success" in data) ||
    typeof data.success !== "boolean"
  ) {
    return false;
  }

  if ("data" in data && data.data !== undefined) {
    if (dataGuard && !dataGuard(data.data)) {
      return false;
    }
  }

  if ("error" in data && data.error !== undefined) {
    if (errorGuard && !errorGuard(data.error)) {
      return false;
    }
  }

  return true;
}

export function isSimpleMessageData(data: unknown): data is SimpleMessageData {
  return (
    typeof data === "object" &&
    data !== null &&
    "message" in data &&
    typeof data.message === "string"
  );
}





