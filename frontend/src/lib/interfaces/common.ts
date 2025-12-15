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

export interface SimpleError {
  message: string | null | undefined;
}

export interface FormError extends SimpleError {
  fields?: FormErrorFields;
}

export type FormErrorFields = Record<string, string | null>;

export function isFormErrorFields(data: unknown): data is FormErrorFields {
  return (
    typeof data === "object" &&
    data !== null &&
    Object.entries(data).every(
      ([key, value]) =>
        typeof key === "string" &&
        (value === null || typeof value === "string"),
    )
  );
}

export function isFormError(data: unknown): data is FormError {
  return (
    typeof data === "object" &&
    data !== null &&
    "message" in data &&
    (data.message === null ||
      data.message === undefined ||
      typeof data.message === "string") &&
    (!("fields" in data) ||
      data.fields === undefined ||
      isFormErrorFields(data.fields))
  );
}

export interface ProcessedResponse<T> {
  statusCode: number;
  jsonData?: T;
}

export async function ProcessResponse<T>(
  response: Response,
  guard: (data: unknown) => data is T,
): Promise<ProcessedResponse<T> | { statusCode: number }> {
  try {
    const jsonData = await response.json();
    if (guard(jsonData)) {
      return {
        statusCode: response.status,
        jsonData,
      };
    }
    return { statusCode: response.status };
  } catch {
    return { statusCode: response.status };
  }
}

export interface SelectOption {
  value: string;
  title: string;
}

export type SelectOptionList = SelectOption[];

export function isSelectOption(data: unknown): data is SelectOption {
  return (
    typeof data === "object" &&
    data !== null &&
    "value" in data &&
    typeof data.value === "string" &&
    "title" in data &&
    typeof data.title === "string"
  );
}

export function isSelectOptionList(data: unknown): data is SelectOptionList {
  return Array.isArray(data) && data.every((item) => isSelectOption(item));
}

export type SelectOptionListResponse = CommonResponse<
  SelectOptionList,
  SimpleError
>;

export function isSelectOptionListResponse(
  data: unknown,
): data is SelectOptionListResponse {
  return isCommonResponse(data, isSelectOptionList, isSimpleError);
}

export interface CommonResponse<T, E> {
  data?: T;
  error?: E;
}

export interface SimpleMessageData {
  message: string;
}

export function isSimpleMessageData(data: unknown): data is SimpleMessageData {
  return (
    typeof data === "object" &&
    data !== null &&
    "message" in data &&
    typeof data.message === "string"
  );
}

export function isSimpleError(data: unknown): data is SimpleError {
  return (
    typeof data === "object" &&
    data !== null &&
    "message" in data &&
    (data.message === null ||
      data.message === undefined ||
      typeof data.message === "string")
  );
}

export function isCommonResponse<T, E>(
  data: unknown,
  dataGuard?: (value: unknown) => value is T,
  errorGuard?: (value: unknown) => value is E,
): data is CommonResponse<T, E> {
  if (typeof data !== "object" || data === null) {
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

export type PagerMeta = {
  page: number;
  limit: number;
  total: number;
};

export function isPagerMeta(data: unknown): data is PagerMeta {
  return (
    typeof data === "object" &&
    data !== null &&
    "page" in data &&
    typeof data.page === "number" &&
    "limit" in data &&
    typeof data.limit === "number" &&
    "total" in data &&
    typeof data.total === "number"
  );
}

export interface PaginatedDataResponse<T, E> {
  meta?: PagerMeta;
  data?: T;
  error?: E;
}

export function isPaginatedDataResponse<T, E>(
  data: unknown,
  dataGuard?: (value: unknown) => value is T,
  errorGuard?: (value: unknown) => value is E,
): data is PaginatedDataResponse<T, E> {
  if (typeof data !== "object" || data === null) {
    return false;
  }

  if ("meta" in data && data.meta !== undefined) {
    if (!isPagerMeta(data.meta)) {
      return false;
    }
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
