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
    act,
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Edit from "./Edit";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    navigate: vi.fn(),
    createResult: null as unknown,
    params: {} as Record<string, string>,
}));
vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => mocks.params,
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/modules/inventory_movements/lib/slice.ts", () => ({
    create: (payload: unknown) => ({ kind: "create", payload }),
    get: (payload: unknown) => ({ kind: "get", payload }),
    select_list: (payload: unknown) => ({ kind: "list", payload }),
}));
vi.mock("@/components/modules/inventory/Edit.tsx", () => ({
    default: ({
        createOnly,
        onSuccess,
    }: {
        createOnly: boolean;
        onSuccess: (value: { id: string }) => void;
    }) => (
        <button
            type="button"
            onClick={() => onSuccess({ id: "new-inventory" })}
        >
            {createOnly ? "Complete inventory" : "Wrong mode"}
        </button>
    ),
}));
const movement = { id: "movement-1", inventory_id: "new-inventory" };
const fields = {
    id: null,
    inventory_id: null,
    movement_type: null,
    quantity: "Quantity required",
    reference_type: null,
    reference_id: null,
    unit_price: null,
    total_price: null,
    tax_id: null,
};

describe("movement editor", () => {
    beforeEach(() => {
        mocks.params = {};
        mocks.navigate.mockReset();
        mocks.createResult = { kind: "success", data: { data: movement } };
        mocks.dispatch
            .mockReset()
            .mockImplementation(
                (action: { kind: string; payload: { list?: string } }) => ({
                    unwrap: async () =>
                        action.kind === "create"
                            ? mocks.createResult
                            : {
                                  kind: "success",
                                  data: {
                                      data:
                                          action.payload.list === "inventory"
                                              ? [
                                                    {
                                                        value: "new-inventory",
                                                        title: "New inventory",
                                                    },
                                                ]
                                              : [],
                                  },
                              },
                }),
            );
    });
    afterEach(cleanup);

    it("refreshes and selects embedded inventory without losing form values, then calls onSuccess", async () => {
        const onSuccess = vi.fn();
        const { container } = render(
            <Edit
                showCard={false}
                onSuccess={onSuccess}
                referenceType="worksheets"
            />,
        );
        fireEvent.change(screen.getByLabelText("Mennyiség"), {
            target: { value: "42" },
        });
        fireEvent.click(
            container.querySelector("button:has(svg.lucide-plus)")!,
        );
        fireEvent.click(
            await screen.findByRole("button", { name: "Complete inventory" }),
        );
        await waitFor(() =>
            expect(screen.queryByRole("dialog")).not.toBeInTheDocument(),
        );
        await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, 0));
        });
        expect(screen.getByLabelText("Mennyiség")).toHaveValue("42,00");
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) =>
                    action.kind === "list" &&
                    action.payload.list === "inventory",
            ),
        ).toHaveLength(2);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(movement));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: {
                input: expect.objectContaining({
                    inventory_id: "new-inventory",
                    quantity: "42",
                    reference_type: "worksheets",
                }),
            },
        });
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("shows and clears backend field errors without navigating", async () => {
        mocks.createResult = {
            kind: "http_error",
            status: 422,
            body: { error: { message: "Invalid", fields } },
        };
        render(<Edit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(
            await screen.findByText("Quantity required"),
        ).toBeInTheDocument();
        expect(screen.getByLabelText("Mennyiség")).toHaveAttribute(
            "aria-invalid",
            "true",
        );
        fireEvent.change(screen.getByLabelText("Mennyiség"), {
            target: { value: "2" },
        });
        expect(screen.queryByText("Quantity required")).not.toBeInTheDocument();
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});
