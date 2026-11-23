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
import { isBase64DataResponse } from "./guards";
import {
    isSelectOption,
    isSelectOptionList,
    isSelectOptionListResponse,
} from "./interface";

describe("shared response guards", () => {
    it("validates select options and their success envelope", () => {
        const option = { value: "id", title: "Name" };
        expect(isSelectOption(option)).toBe(true);
        expect(isSelectOption({ value: "id", title: null })).toBe(false);
        expect(isSelectOptionList([option])).toBe(true);
        expect(isSelectOptionList([option, {}])).toBe(false);
        expect(isSelectOptionListResponse({ meta: null, data: [option] })).toBe(
            true,
        );
        expect(isSelectOptionListResponse({ data: [] })).toBe(true);
        expect(
            isSelectOptionListResponse({ error: { message: "error" } }),
        ).toBe(false);
        expect(isSelectOptionListResponse({ data: null })).toBe(false);
    });

    it("requires a string for base64 success responses", () => {
        expect(isBase64DataResponse({ meta: null, data: "YWJj" })).toBe(true);
        expect(isBase64DataResponse({ data: "" })).toBe(true);
        for (const value of [
            null,
            {},
            { data: null },
            { data: 1 },
            { error: {} },
            { data: "ok", error: {} },
        ]) {
            expect(isBase64DataResponse(value)).toBe(false);
        }
    });
});
