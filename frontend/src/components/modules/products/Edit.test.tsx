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
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Edit from "./Edit";

const mocks = vi.hoisted(() => ({
    id: undefined as string | undefined,
    dispatch: vi.fn(),
    navigate: vi.fn(),
    toast: vi.fn(),
    abort: vi.fn(),
}));
vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: mocks.id }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/modules/products/lib/slice.ts", () => ({
    create: (payload: unknown) => ({ kind: "create", payload }),
    update: (payload: unknown) => ({ kind: "update", payload }),
    get: (payload: unknown) => ({ kind: "get", payload }),
    select_list: (payload: unknown) => ({ kind: "list", payload }),
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: mocks.toast } }));

const product = {
    id: "product-id",
    name: "Existing product",
    description: "Details",
    unit_of_measure_id: "unit-id",
    status: "active",
    created_by_id: "user",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
    deleted_at: null,
};
const fields = {
    id: null,
    name: "Required",
    description: null,
    unit_of_measure_id: null,
    new_unit_of_measure: null,
    status: null,
};

describe("product editor", () => {
    beforeEach(() => {
        mocks.id = undefined;
        vi.clearAllMocks();
        mocks.dispatch.mockImplementation((action: { kind: string }) => ({
            unwrap: async () => ({
                kind: "success",
                data: {
                    data:
                        action.kind === "list"
                            ? [{ value: "unit-id", title: "Box" }]
                            : product,
                },
            }),
            abort: mocks.abort,
        }));
    });
    afterEach(cleanup);

    it("creates in a parent dialog without using the parent route id", async () => {
        mocks.id = "parent-id";
        const onSuccess = vi.fn();
        render(<Edit createOnly showCard={false} onSuccess={onSuccess} />);
        fireEvent.change(screen.getByLabelText("Név"), {
            target: { value: "New product" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(product));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: {
                input: expect.objectContaining({
                    id: null,
                    name: "New product",
                }),
            },
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "get" }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("loads and updates an existing product", async () => {
        mocks.id = "product-id";
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Név")).toHaveValue(
                "Existing product",
            ),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "get",
            payload: { uuid: "product-id" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/termek/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "update",
            payload: {
                input: expect.objectContaining({
                    id: "product-id",
                    unit_of_measure_id: "unit-id",
                }),
            },
        });
    });

    it("shows and clears backend field errors", async () => {
        const originalDispatch = mocks.dispatch.getMockImplementation()!;
        mocks.dispatch.mockImplementation((action: { kind: string }) =>
            action.kind === "create"
                ? {
                      unwrap: async () => ({
                          kind: "http_error",
                          status: 422,
                          body: { error: { message: "Invalid", fields } },
                      }),
                      abort: mocks.abort,
                  }
                : originalDispatch(action),
        );
        render(<Edit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(await screen.findByText("Required")).toBeInTheDocument();
        fireEvent.change(screen.getByLabelText("Név"), {
            target: { value: "Valid" },
        });
        expect(screen.queryByText("Required")).not.toBeInTheDocument();
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});
