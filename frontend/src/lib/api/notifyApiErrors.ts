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
import { toast } from "@/components/ui/toast-lib";

/** Shows an error toast for any non-success result. Returns true if the result failed. */
export function notifyApiError(result: ApiResult<unknown, unknown>): boolean {
    if (result.kind === "success") return false;
    const message = getErrorMessage(result);
    if (message !== null) {
        // null = aborted, stay silent
        toast.add({
            type: "error",
            title: "Hiba történt",
            description: message,
        });
    }
    return true;
}
