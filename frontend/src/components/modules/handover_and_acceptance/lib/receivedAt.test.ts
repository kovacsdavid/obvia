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

import { describe, expect, it } from "vitest";
import { formatReceiptTime, resolveReceiptTime } from "./receivedAt";

describe("receipt time conversion", () => {
    it("uses the account timezone with seasonal offsets and date boundaries", () => {
        expect(
            resolveReceiptTime("2026-01-02T00:30", "Europe/Budapest"),
        ).toEqual({
            kind: "valid",
            options: [{ utc: "2026-01-01T23:30:00.000Z", offset: "+01:00" }],
        });
        expect(
            resolveReceiptTime("2026-07-02T12:30", "Europe/Budapest"),
        ).toEqual({
            kind: "valid",
            options: [{ utc: "2026-07-02T10:30:00.000Z", offset: "+02:00" }],
        });
        expect(
            formatReceiptTime("2026-01-01T23:30:45.123Z", "Europe/Budapest"),
        ).toBe("2026-01-02T00:30:45.123");
    });

    it("rejects invalid timezones, invalid times, and daylight-saving gaps", () => {
        expect(resolveReceiptTime("2026-01-02T12:00", undefined).kind).toBe(
            "invalid-zone",
        );
        expect(
            resolveReceiptTime("2026-01-02T12:00", "invalid/timezone").kind,
        ).toBe("invalid-zone");
        expect(resolveReceiptTime("", "Europe/Budapest").kind).toBe(
            "invalid-time",
        );
        expect(
            resolveReceiptTime("2026-02-30T12:00", "Europe/Budapest").kind,
        ).toBe("invalid-time");
        expect(
            resolveReceiptTime("2026-03-29T02:30", "Europe/Budapest").kind,
        ).toBe("gap");
    });

    it("offers both instants in a daylight-saving overlap", () => {
        const result = resolveReceiptTime(
            "2026-10-25T02:30",
            "Europe/Budapest",
        );
        expect(result.kind).toBe("valid");
        if (result.kind !== "valid") throw new Error("Expected valid time");
        expect(result.options).toEqual(
            expect.arrayContaining([
                { utc: "2026-10-25T00:30:00.000Z", offset: "+02:00" },
                { utc: "2026-10-25T01:30:00.000Z", offset: "+01:00" },
            ]),
        );
        expect(result.options).toHaveLength(2);
    });
});
