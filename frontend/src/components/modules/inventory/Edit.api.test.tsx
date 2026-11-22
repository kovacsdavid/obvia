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
    within,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import Edit from "./Edit";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    navigate: vi.fn(),
    toast: vi.fn(),
}));
vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: "parent" }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: mocks.toast } }));
vi.mock("@/components/modules/inventory/lib/slice.ts", () => {
    const action = (kind: string) =>
        Object.assign((payload: unknown) => ({ kind, payload }), {
            fulfilled: {
                match: (response: { kind: string }) => response.kind === kind,
            },
        });
    return {
        create: action("create"),
        update: action("update"),
        get: action("get"),
        select_list: action("select"),
    };
});
vi.mock("@/components/modules/products/Edit.tsx", () => ({
    default: ({
        onSuccess,
    }: {
        onSuccess: (value: { id: string }) => void;
    }) => (
        <button onClick={() => onSuccess({ id: "new-product" })}>
            Complete product
        </button>
    ),
}));
vi.mock("@/components/modules/warehouses/Edit.tsx", () => ({
    default: ({
        onSuccess,
    }: {
        onSuccess: (value: { id: string }) => void;
    }) => (
        <button onClick={() => onSuccess({ id: "new-warehouse" })}>
            Complete warehouse
        </button>
    ),
}));

const inventory = { id: "inventory-id" };
const lists: Record<string, { value: string; title: string }[]> = {
    products: [{ value: "new-product", title: "New product" }],
    warehouses: [{ value: "new-warehouse", title: "New warehouse" }],
    currencies: [{ value: "HUF", title: "Forint" }],
};
type Action = { kind: string; payload: { list?: string } };
beforeEach(() => {
    vi.clearAllMocks();
    mocks.dispatch.mockImplementation((action: Action) =>
        Promise.resolve({
            kind: action.kind,
            payload: {
                kind: "success",
                status: 200,
                data: {
                    data:
                        action.kind === "select"
                            ? lists[action.payload.list ?? ""]
                            : inventory,
                },
            },
        }),
    );
});
afterEach(cleanup);

it("creates embedded inventory and returns the created record", async () => {
    const onSuccess = vi.fn();
    render(<Edit createOnly onSuccess={onSuccess} />);
    fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
    await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(inventory));
    expect(mocks.dispatch).toHaveBeenCalledWith({
        kind: "create",
        payload: { input: expect.objectContaining({ id: null }) },
    });
    expect(mocks.navigate).not.toHaveBeenCalled();
});

it("shows backend field errors without navigating", async () => {
    const original = mocks.dispatch.getMockImplementation()!;
    mocks.dispatch.mockImplementation((action: Action) =>
        action.kind === "create"
            ? Promise.resolve({
                  kind: "create",
                  payload: {
                      kind: "http_error",
                      status: 422,
                      body: {
                          error: {
                              message: "Invalid",
                              fields: {
                                  id: null,
                                  product_id: "Choose a product",
                                  warehouse_id: null,
                                  minimum_stock: null,
                                  maximum_stock: null,
                                  currency_code: null,
                                  status: null,
                              },
                          },
                      },
                  },
              })
            : original(action),
    );
    render(<Edit createOnly />);
    fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
    expect(await screen.findByText("Choose a product")).toBeVisible();
    expect(mocks.navigate).not.toHaveBeenCalled();
});

it.each([
    ["Termék létrehozása", "Complete product", "New product", 0],
    ["Raktár létrehozása", "Complete warehouse", "New warehouse", 1],
])(
    "refreshes lists and selects newly created %s",
    async (dialog, button, selected, index) => {
        const { container } = render(<Edit createOnly />);
        await waitFor(() => expect(mocks.dispatch).toHaveBeenCalledTimes(3));
        const buttons = container.querySelectorAll(
            "button:has(svg.lucide-plus)",
        );
        fireEvent.click(buttons[index]);
        fireEvent.click(
            within(screen.getByRole("dialog", { name: dialog })).getByText(
                button,
            ),
        );
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", { name: dialog }),
            ).not.toBeInTheDocument(),
        );
        await waitFor(() =>
            expect(
                screen.getByText(selected, {
                    selector: "span[data-slot='select-value']",
                }),
            ).toBeVisible(),
        );
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) => action.kind === "select",
            ),
        ).toHaveLength(6);
    },
);
