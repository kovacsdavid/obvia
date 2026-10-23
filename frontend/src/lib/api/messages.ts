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

const statusMessages: Record<number, string> = {
    400: "Hibás kérés. Kérlek, ellenőrizd a megadott adatokat!",
    401: "A munkameneted lejárt. Kérlek, jelentkezz be újra!",
    403: "Nincs jogosultságod ehhez a művelethez.",
    404: "A keresett erőforrás nem található.",
    408: "A szerver nem kapott időben választ. Kérlek, próbáld újra!",
    409: "Ütközés történt: az adatot időközben valaki módosította.",
    413: "A küldött adat túl nagy.",
    415: "Nem támogatott adatformátum.",
    422: "Kérlek, javítsd a hibásan kitöltött mezőket!",
    500: "Belső szerverhiba. Kérlek, próbáld újra később!",
    502: "A szerver jelenleg nem érhető el (502). Próbáld újra később!",
    503: "A szolgáltatás átmenetileg nem elérhető. Próbáld újra később!",
    504: "A szerver nem válaszolt időben (504). Próbáld újra később!",
};

export function getErrorMessage(
    result: Exclude<ApiResult<unknown, unknown>, { kind: "success" }>,
): string | null {
    switch (result.kind) {
        case "aborted":
            return null;
        case "timeout":
            return "A kérés túllépte az időkorlátot. Ellenőrizd a kapcsolatot, majd próbáld újra!";
        case "network_error":
            return result.offline
                ? "Nincs internetkapcsolat. Csatlakozz a hálózathoz, majd próbáld újra!"
                : "Nem sikerült kapcsolódni a szerverhez. Kérlek, próbáld újra!";
        case "rate_limited":
            return result.retryAfterSeconds !== undefined
                ? `Túl sok kérés. Kérlek, várj ${result.retryAfterSeconds} másodpercet!`
                : "Túl sok kérés. Kérlek, várj egy kicsit, majd próbáld újra!";
        case "invalid_response":
            return "A szerver váratlan formátumú választ küldött.";
        case "http_error":
            return (
                result.body?.error.message ??
                statusMessages[result.status] ??
                (result.status >= 500
                    ? `Szerverhiba (${result.status}). Próbáld újra később!`
                    : `A kérés sikertelen (${result.status}).`)
            );
    }
}
