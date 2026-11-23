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

import { renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { useSelectList } from "./use_select_list";

describe("useSelectList", () => {
    it("sets valid success data", () => {
        const { result } = renderHook(() => useSelectList());
        const setList = vi.fn();
        const setError = vi.fn();
        const data = [{ value: "id", title: "Name" }];

        result.current.setListResponse(
            { statusCode: 200, jsonData: { meta: null, data } },
            setList,
            setError,
        );

        expect(setList).toHaveBeenCalledWith(data);
        expect(setError).not.toHaveBeenCalled();
    });

    it("extracts an error body for non-success responses", () => {
        const { result } = renderHook(() => useSelectList());
        const setList = vi.fn();
        const setError = vi.fn();

        result.current.setListResponse(
            { statusCode: 422, jsonData: { error: { message: "Invalid" } } },
            setList,
            setError,
        );

        expect(setList).not.toHaveBeenCalled();
        expect(setError).toHaveBeenCalledWith({
            message: "Invalid",
            fields: {},
        });
    });

    it.each([
        { statusCode: 200 },
        { statusCode: 200, jsonData: { error: { message: "Invalid" } } },
        { statusCode: 200, jsonData: { data: [{ value: 1, title: "Name" }] } },
        { statusCode: 500 },
        { statusCode: 500, jsonData: { error: { message: 42 } } },
        { statusCode: 500, jsonData: { error: { message: null } } },
        { statusCode: 500, jsonData: { data: [] } },
    ])("reports unexpected responses: %o", (payload) => {
        const { result } = renderHook(() => useSelectList());
        const setList = vi.fn();
        const setError = vi.fn();

        result.current.setListResponse(payload, setList, setError);

        expect(setList).not.toHaveBeenCalled();
        expect(setError).toHaveBeenCalledWith({
            message: "Váratlan hiba történt a feldolgozás során!",
            fields: {},
        });
    });
});
