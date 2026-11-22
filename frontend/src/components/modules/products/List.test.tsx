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

import {
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { MemoryRouter } from "react-router";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { decode_url_encoded_get_query } from "@/lib/get_query";
import List from "./List";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    list: vi.fn(),
    deleteItem: vi.fn(),
    print: vi.fn(),
    openPopup: vi.fn(),
    updateWindowWithPdfData: vi.fn(),
    close: vi.fn(),
    abort: vi.fn(),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/products/lib/slice.ts", () => ({
    list: (payload: unknown) => ({ kind: "list", payload }),
    deleteItem: (payload: unknown) => ({ kind: "deleteItem", payload }),
    print: (payload: unknown) => ({ kind: "print", payload }),
}));
vi.mock("@/lib/utils.ts", async (importOriginal) => ({
    ...(await importOriginal<typeof import("@/lib/utils.ts")>()),
    openPopup: mocks.openPopup,
    updateWindowWithPdfData: mocks.updateWindowWithPdfData,
}));
const product = {
    id: "product-1",
    name: "Product",
    description: "Details",
    unit_of_measure: "Box",
    status: "active",
    created_by: "Anna",
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
};
const listResult = (page: number, data = [product]) => ({
    kind: "success",
    data: { meta: { page, limit: 25, total: 50 }, data },
});
async function renderList() {
    render(
        <MemoryRouter>
            <List />
        </MemoryRouter>,
    );
    await screen.findByText("Product");
}
async function chooseAction(name: string) {
    fireEvent.pointerDown(
        screen.getByRole("button", { name: "Menü megnyitása" }),
        { button: 0, ctrlKey: false },
    );
    fireEvent.click(await screen.findByRole("menuitem", { name }));
}
describe("product list", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.list.mockResolvedValue(listResult(1));
        mocks.deleteItem.mockResolvedValue({ kind: "success", data: {} });
        mocks.openPopup.mockReturnValue({ close: mocks.close });
        mocks.dispatch.mockImplementation(
            (action: {
                kind: "list" | "deleteItem" | "print";
                payload: unknown;
            }) => ({
                unwrap: () => mocks[action.kind](action.payload),
                abort: mocks.abort,
            }),
        );
    });
    afterEach(cleanup);

    it("uses pagination metadata and refreshes the current query after deletion", async () => {
        mocks.list
            .mockResolvedValueOnce(listResult(1))
            .mockResolvedValueOnce(listResult(2))
            .mockResolvedValueOnce(listResult(2, []));
        await renderList();
        fireEvent.click(screen.getByText("2"));
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(2));
        const query = mocks.list.mock.calls[1][0].query;
        expect(decode_url_encoded_get_query(query).paging).toEqual({
            page: 2,
            limit: 25,
        });
        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(3));
        expect(mocks.deleteItem).toHaveBeenCalledWith({ uuid: "product-1" });
        expect(mocks.list).toHaveBeenLastCalledWith({ query });
        await waitFor(() =>
            expect(screen.queryByText("Product")).not.toBeInTheDocument(),
        );
    });

    it("displays a successful PDF", async () => {
        const pdf = new Uint8Array([37, 80, 68, 70]);
        mocks.print.mockResolvedValue({ kind: "success", data: pdf });
        await renderList();
        await chooseAction("Nyomtatás");
        await waitFor(() =>
            expect(mocks.updateWindowWithPdfData).toHaveBeenCalledWith(
                mocks.openPopup.mock.results[0].value,
                pdf,
            ),
        );
        expect(mocks.print).toHaveBeenCalledWith({ uuid: "product-1" });
    });

    it.each([
        ["API error", () => Promise.resolve({ kind: "network_error" })],
        ["empty PDF", () => Promise.resolve({ kind: "success", data: null })],
        ["rejection", () => Promise.reject(new Error("failed"))],
    ])("closes the print popup on %s", async (_label, result) => {
        mocks.print.mockImplementation(result);
        await renderList();
        await chooseAction("Nyomtatás");
        await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
        expect(mocks.updateWindowWithPdfData).not.toHaveBeenCalled();
    });
});
