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
}));
vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: mocks.id }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: mocks.toast } }));
vi.mock("@/components/modules/warehouses/lib/slice.ts", () => ({
    create: (payload: unknown) => ({ kind: "create", payload }),
    update: (payload: unknown) => ({ kind: "update", payload }),
    get: (payload: unknown) => ({ kind: "get", payload }),
}));
const warehouse = {
    id: "warehouse-id",
    name: "Existing",
    contact_name: "Alice",
    contact_phone: "123",
    status: "active",
    created_by_id: "user",
    created_at: "date",
    updated_at: "date",
    deleted_at: null,
};

describe("warehouse editor", () => {
    beforeEach(() => {
        mocks.id = undefined;
        mocks.dispatch.mockReset();
        mocks.navigate.mockReset();
        mocks.toast.mockReset();
        mocks.dispatch.mockImplementation(() => ({
            unwrap: async () => ({
                kind: "success",
                data: { data: warehouse },
            }),
        }));
    });
    afterEach(cleanup);

    it("creates inside a parent route and invokes its callback", async () => {
        mocks.id = "parent-id";
        const onSuccess = vi.fn();
        render(<Edit createOnly onSuccess={onSuccess} />);
        fireEvent.change(screen.getByLabelText("Név"), {
            target: { value: "New" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(warehouse));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: {
                input: {
                    id: null,
                    name: "New",
                    contactName: "",
                    contactPhone: "",
                    status: "active",
                },
            },
        });
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("loads and updates a warehouse", async () => {
        mocks.id = "warehouse-id";
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Név")).toHaveValue("Existing"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "get",
            payload: { uuid: "warehouse-id" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/raktar/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "update",
            payload: {
                input: {
                    id: "warehouse-id",
                    name: "Existing",
                    contactName: "Alice",
                    contactPhone: "123",
                    status: "active",
                },
            },
        });
    });

    it("renders and clears backend field errors", async () => {
        mocks.dispatch.mockReturnValue({
            unwrap: async () => ({
                kind: "http_error",
                status: 422,
                body: {
                    error: {
                        message: "Invalid",
                        fields: {
                            id: null,
                            name: "Name required",
                            contact_name: null,
                            contact_phone: null,
                            status: null,
                        },
                    },
                },
            }),
        });
        render(<Edit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(await screen.findByText("Name required")).toBeInTheDocument();
        fireEvent.change(screen.getByLabelText("Név"), {
            target: { value: "Valid" },
        });
        expect(screen.queryByText("Name required")).not.toBeInTheDocument();
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});
