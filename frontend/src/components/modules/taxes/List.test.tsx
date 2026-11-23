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
    updatePdf: vi.fn(),
    close: vi.fn(),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/taxes/lib/slice.ts", () => ({
    list: (payload: unknown) => ({ kind: "list", payload }),
    deleteItem: (payload: unknown) => ({ kind: "deleteItem", payload }),
    print: (payload: unknown) => ({ kind: "print", payload }),
}));
vi.mock("@/lib/utils.ts", async (original) => ({
    ...(await original<typeof import("@/lib/utils.ts")>()),
    openPopup: mocks.openPopup,
    updateWindowWithPdfData: mocks.updatePdf,
}));

const tax = {
    id: "tax-1",
    rate: "27",
    description: "Standard VAT",
    country: "Hungary",
    tax_category: "AK",
    is_rate_applicable: true,
    legal_text: null,
    status: "active",
    created_by: "Anna",
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
};
async function showList() {
    render(
        <MemoryRouter>
            <List />
        </MemoryRouter>,
    );
    await screen.findByText("Standard VAT");
}
async function chooseAction(name: string) {
    fireEvent.pointerDown(
        screen.getByRole("button", { name: "Menü megnyitása" }),
        { button: 0, ctrlKey: false },
    );
    fireEvent.click(await screen.findByRole("menuitem", { name }));
}

describe("tax list", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.list.mockResolvedValue({
            kind: "success",
            data: { data: [tax], meta: { page: 1, limit: 25, total: 50 } },
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

    it("loads pagination data and refreshes after a successful delete", async () => {
        await showList();
        expect(mocks.list).toHaveBeenCalledWith({ query: null });
        expect(screen.getByText("2")).toBeInTheDocument();
        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(2));
        expect(mocks.deleteItem).toHaveBeenCalledWith({ uuid: "tax-1" });
    });

    it("displays returned PDF bytes", async () => {
        const pdf = new Uint8Array([37, 80, 68, 70]);
        mocks.print.mockResolvedValue({ kind: "success", data: pdf });
        await showList();
        await chooseAction("Nyomtatás");
        await waitFor(() =>
            expect(mocks.updatePdf).toHaveBeenCalledWith(
                mocks.openPopup.mock.results[0].value,
                pdf,
            ),
        );
        expect(mocks.print).toHaveBeenCalledWith({ uuid: "tax-1" });
        expect(mocks.close).not.toHaveBeenCalled();
    });

    it.each([
        ["API error", () => Promise.resolve({ kind: "network_error" })],
        ["empty PDF", () => Promise.resolve({ kind: "success", data: null })],
        ["rejection", () => Promise.reject(new Error("failed"))],
    ])("closes the popup on %s", async (_label, result) => {
        mocks.print.mockImplementation(result);
        await showList();
        await chooseAction("Nyomtatás");
        await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
        expect(mocks.updatePdf).not.toHaveBeenCalled();
    });
});
