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
import List from "./List";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    list: vi.fn(),
    deleteItem: vi.fn(),
    print: vi.fn(),
    openPopup: vi.fn(),
    updateWindowWithPdfData: vi.fn(),
    close: vi.fn(),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/warehouses/lib/slice.ts", () => ({
    list: (payload: unknown) => ({ kind: "list", payload }),
    deleteItem: (payload: unknown) => ({ kind: "deleteItem", payload }),
    print: (payload: unknown) => ({ kind: "print", payload }),
}));
vi.mock("@/lib/utils.ts", async (original) => ({
    ...(await original<typeof import("@/lib/utils.ts")>()),
    openPopup: mocks.openPopup,
    updateWindowWithPdfData: mocks.updateWindowWithPdfData,
}));
const warehouse = {
    id: "warehouse-id",
    name: "Main warehouse",
    contact_name: "Alice",
    contact_phone: "123",
    status: "active",
    created_by: "Anna",
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
};

async function renderList() {
    render(
        <MemoryRouter>
            <List />
        </MemoryRouter>,
    );
    await screen.findByText("Main warehouse");
}
async function chooseAction(name: string) {
    fireEvent.pointerDown(
        screen.getByRole("button", { name: "Menü megnyitása" }),
        { button: 0, ctrlKey: false },
    );
    fireEvent.click(await screen.findByRole("menuitem", { name }));
}
describe("warehouse list", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.list.mockResolvedValue({
            kind: "success",
            data: { meta: { page: 1, limit: 25, total: 1 }, data: [warehouse] },
        });
        mocks.deleteItem.mockResolvedValue({ kind: "success", data: {} });
        mocks.openPopup.mockReturnValue({ close: mocks.close });
        mocks.dispatch.mockImplementation(
            (action: {
                kind: "list" | "deleteItem" | "print";
                payload: unknown;
            }) => ({ unwrap: () => mocks[action.kind](action.payload) }),
        );
    });
    afterEach(cleanup);

    it("loads and refreshes the list after deletion", async () => {
        await renderList();
        expect(mocks.list).toHaveBeenCalledWith({ query: null });
        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(2));
        expect(mocks.deleteItem).toHaveBeenCalledWith({ uuid: "warehouse-id" });
    });
    it("does not refresh after a failed deletion", async () => {
        mocks.deleteItem.mockResolvedValue({
            kind: "network_error",
            offline: false,
        });
        await renderList();
        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.deleteItem).toHaveBeenCalledOnce());
        expect(mocks.list).toHaveBeenCalledOnce();
    });
    it("does not refresh after a rejected deletion", async () => {
        mocks.deleteItem.mockRejectedValue(new Error("delete failed"));
        await renderList();
        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.deleteItem).toHaveBeenCalledOnce());
        expect(mocks.list).toHaveBeenCalledOnce();
    });
    it("does not request a PDF when the popup is blocked", async () => {
        mocks.openPopup.mockReturnValue(null);
        await renderList();
        await chooseAction("Nyomtatás");
        expect(mocks.print).not.toHaveBeenCalled();
    });
    it("opens the returned PDF", async () => {
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
        expect(mocks.print).toHaveBeenCalledWith({ uuid: "warehouse-id" });
        expect(mocks.close).not.toHaveBeenCalled();
    });
    it.each([
        [
            "API failure",
            () => Promise.resolve({ kind: "network_error", offline: false }),
        ],
        [
            "empty result",
            () => Promise.resolve({ kind: "success", data: null }),
        ],
        ["rejection", () => Promise.reject(new Error("failed"))],
    ])("closes the PDF popup on %s", async (_label, result) => {
        mocks.print.mockImplementation(result);
        await renderList();
        await chooseAction("Nyomtatás");
        await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
        expect(mocks.updateWindowWithPdfData).not.toHaveBeenCalled();
    });
});
