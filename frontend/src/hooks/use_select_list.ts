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
  ProcessedResponse,
  SelectOptionList,
  SelectOptionListResponse,
} from "@/lib/interfaces/common.ts";
import React, { useCallback } from "react";

export function useSelectList() {
  const setListResponse = useCallback(
    (
      payload: ProcessedResponse<SelectOptionListResponse>,
      setList: (data: React.SetStateAction<SelectOptionList>) => void,
      setError: (data: React.SetStateAction<FormError | null>) => void,
    ) => {
      if (payload.statusCode === 200) {
        if (typeof payload.jsonData.data !== "undefined") {
          setList(payload.jsonData.data);
        }
      } else if (typeof payload.jsonData?.error !== "undefined") {
        if (typeof payload.jsonData.error !== "undefined") {
          setError({
            message: payload.jsonData.error.message,
            fields: {},
          });
        }
      } else {
        setError({
          message: "Váratlan hiba történt a feldolgozás során!",
          fields: {},
        });
      }
    },
    [],
  );

  return {
    setListResponse,
  };
}
