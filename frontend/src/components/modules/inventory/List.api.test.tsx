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
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import List from "./List";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    open: vi.fn(),
    display: vi.fn(),
    close: vi.fn(),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/inventory/lib/slice.ts", () => {
    const action = (kind: string) =>
        Object.assign((payload: unknown) => ({ kind, payload }), {
            fulfilled: {
                match: (response: { kind: string }) => response.kind === kind,
            },
        });
    return {
        list: action("list"),
        deleteItem: action("delete"),
        print: action("print"),
    };
});
vi.mock("@/lib/utils.ts", async (original) => ({
    ...(await original<typeof import("@/lib/utils.ts")>()),
    openPopup: mocks.open,
    updateWindowWithPdfData: mocks.display,
}));

const record = {
    id: "inventory-id",
    product: "Product A",
    warehouse: "Warehouse A",
    quantity_on_hand: "10",
    quantity_reserved: "2",
    quantity_available: "8",
    status: "active",
    currency: "Forint",
    created_by: "Anna",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
    deleted_at: null,
};
let printResult: unknown;
beforeEach(() => {
    vi.clearAllMocks();
    printResult = {
        kind: "success",
        status: 200,
        data: new Uint8Array([37, 80]),
    };
    mocks.open.mockReturnValue({ close: mocks.close });
    mocks.dispatch.mockImplementation((action: { kind: string }) =>
        Promise.resolve({
            kind: action.kind,
            payload:
                action.kind === "list"
                    ? {
                          kind: "success",
                          status: 200,
                          data: {
                              data: [record],
                              meta: { page: 1, limit: 25, total: 1 },
                          },
                      }
                    : action.kind === "print"
                      ? printResult
                      : { kind: "success", status: 200, data: {} },
        }),
    );
});
afterEach(cleanup);

async function chooseAction(name: string) {
    render(
        <MemoryRouter>
            <List />
        </MemoryRouter>,
    );
    await screen.findByText("Product A");
    fireEvent.pointerDown(
        screen.getByRole("button", { name: "Menü megnyitása" }),
        { button: 0, ctrlKey: false },
    );
    fireEvent.click(await screen.findByRole("menuitem", { name }));
}

it("refreshes the list after deleting inventory", async () => {
    await chooseAction("Törlés");
    await waitFor(() =>
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) => action.kind === "list",
            ),
        ).toHaveLength(2),
    );
    expect(mocks.dispatch).toHaveBeenCalledWith({
        kind: "delete",
        payload: { uuid: "inventory-id" },
    });
});

it("displays successful PDF data in the popup", async () => {
    await chooseAction("Nyomtatás");
    await waitFor(() =>
        expect(mocks.display).toHaveBeenCalledWith(
            mocks.open.mock.results[0].value,
            new Uint8Array([37, 80]),
        ),
    );
    expect(mocks.close).not.toHaveBeenCalled();
});

it.each([
    ["API failure", { kind: "network_error", offline: false }],
    ["empty PDF", { kind: "success", data: null }],
])("closes the popup after %s", async (_label, result) => {
    printResult = result;
    await chooseAction("Nyomtatás");
    await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
    expect(mocks.display).not.toHaveBeenCalled();
});

it("closes the popup if PDF display throws", async () => {
    mocks.display.mockImplementationOnce(() => {
        throw new Error("display failed");
    });
    await chooseAction("Nyomtatás");
    await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
});
