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
    FormError,
    ProcessedJsonResponse,
    SelectOptionList,
} from "@/lib/interface.ts";
import { isSelectOptionListResponse, isSimpleError } from "@/lib/interface.ts";
import React, { useCallback } from "react";

export function useSelectList() {
    const setListResponse = useCallback(
        (
            payload: ProcessedJsonResponse<unknown>,
            setList: (data: React.SetStateAction<SelectOptionList>) => void,
            setError: (data: React.SetStateAction<FormError | null>) => void,
        ) => {
            if (
                payload.statusCode === 200 &&
                isSelectOptionListResponse(payload.jsonData)
            ) {
                setList(payload.jsonData.data);
                return;
            }

            const body = payload.jsonData;
            const error =
                payload.statusCode !== 200 &&
                typeof body === "object" &&
                body !== null &&
                "error" in body &&
                isSimpleError(body.error)
                    ? body.error
                    : null;
            setError({
                message:
                    error?.message ??
                    "Váratlan hiba történt a feldolgozás során!",
                fields: {},
            });
        },
        [],
    );

    return {
        setListResponse,
    };
}
