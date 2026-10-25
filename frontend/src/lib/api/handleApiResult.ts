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

import type { ApiResult } from "@/lib/api/types";
import { getErrorMessage } from "@/lib/api/messages";

export interface ApiResultHandlers<T, E> {
    onSuccess?: (data: T) => void;
    /** Called when the backend returns validation details (e.g. 422) */
    onFieldErrors?: (fields: E) => void;
    /** Shows a generic error message */
    onError?: (message: string) => void;
}

export function handleApiResult<T, E>(
    result: ApiResult<T, E>,
    { onSuccess, onFieldErrors, onError }: ApiResultHandlers<T, E>,
): void {
    if (result.kind === "success") {
        if (typeof onSuccess !== "undefined") onSuccess(result.data);
        return;
    }
    const fields =
        result.kind === "http_error" ? result.body?.error.fields : undefined;
    if (fields !== undefined && onFieldErrors) {
        onFieldErrors(fields);
    }
    const message = getErrorMessage(result);
    if (message !== null && typeof onError !== "undefined") onError(message); // null = aborted
}
