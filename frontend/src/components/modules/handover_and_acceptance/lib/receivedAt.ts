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

import { DateTime, IANAZone } from "luxon";

export type ReceiptTimeResult =
    | { kind: "invalid-zone" }
    | { kind: "invalid-time" }
    | { kind: "gap" }
    | { kind: "valid"; options: { utc: string; offset: string }[] };

export function validReceiptTimeZone(zone: string | undefined): zone is string {
    return !!zone && IANAZone.isValidZone(zone);
}

export function formatReceiptTime(
    utc: string,
    zone: string | undefined,
): string {
    if (!validReceiptTimeZone(zone)) return "";
    const date = DateTime.fromISO(utc, { setZone: true }).setZone(zone);
    return date.isValid ? date.toFormat("yyyy-MM-dd'T'HH:mm:ss.SSS") : "";
}

export function resolveReceiptTime(
    value: string,
    zone: string | undefined,
): ReceiptTimeResult {
    if (!validReceiptTimeZone(zone)) return { kind: "invalid-zone" };
    if (
        !/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(?::\d{2}(?:\.\d{1,3})?)?$/.test(value)
    ) {
        return { kind: "invalid-time" };
    }
    const wall = DateTime.fromISO(value, { zone: "utc" });
    const local = DateTime.fromISO(value, { zone });
    if (!wall.isValid || !local.isValid) return { kind: "invalid-time" };
    if (
        wall.toFormat("yyyy-MM-dd'T'HH:mm:ss.SSS") !==
        local.toFormat("yyyy-MM-dd'T'HH:mm:ss.SSS")
    ) {
        return { kind: "gap" };
    }
    const options = local.getPossibleOffsets().map((date) => ({
        utc: date.toUTC().toISO()!,
        offset: date.toFormat("ZZ"),
    }));
    return { kind: "valid", options };
}
