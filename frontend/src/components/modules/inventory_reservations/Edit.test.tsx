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
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Edit from "./Edit";

const mocks = vi.hoisted(() => ({
    params: {} as { id?: string; inventoryId?: string },
    dispatch: vi.fn(),
    navigate: vi.fn(),
    toast: vi.fn(),
    create: vi.fn(),
    list: vi.fn(),
    get: vi.fn(),
}));
vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => mocks.params,
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/modules/inventory_reservations/lib/slice.ts", () => ({
    create: (payload: unknown) => ({ kind: "create", payload }),
    get: (payload: unknown) => ({ kind: "get", payload }),
    select_list: (payload: unknown) => ({ kind: "list", payload }),
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: mocks.toast } }));
vi.mock("@/components/modules/inventory/Edit.tsx", () => ({
    default: ({
        createOnly,
        showCard,
        onSuccess,
    }: {
        createOnly: boolean;
        showCard: boolean;
        onSuccess: (value: { id: string }) => void;
    }) => (
        <button
            type="button"
            onClick={() => onSuccess({ id: "new-inventory" })}
        >
            {createOnly && !showCard ? "Create inventory" : "Wrong mode"}
        </button>
    ),
}));
const fields = {
    id: null,
    inventory_id: null,
    quantity: "Invalid quantity",
    reference_type: null,
    reference_id: null,
    reserved_until: null,
    status: null,
};

describe("reservation editor", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.params = {};
        mocks.list.mockResolvedValue({
            kind: "success",
            data: {
                data: [{ value: "new-inventory", title: "New inventory" }],
            },
        });
        mocks.get.mockResolvedValue({
            kind: "success",
            data: {
                data: {
                    inventory_id: "old",
                    quantity: "3",
                    reference_type: "worksheets",
                    reference_id: "ref",
                    reserved_until: "2027-01-01",
                    status: "active",
                },
            },
        });
        mocks.create.mockResolvedValue({
            kind: "success",
            status: 201,
            data: {},
        });
        mocks.dispatch.mockImplementation(
            (action: {
                kind: "create" | "get" | "list";
                payload: unknown;
            }) => ({ unwrap: () => mocks[action.kind](action.payload) }),
        );
    });
    afterEach(cleanup);

    it("shows and clears backend field errors without navigating", async () => {
        mocks.create.mockResolvedValue({
            kind: "http_error",
            status: 422,
            body: { error: { message: "Invalid", fields } },
        });
        render(<Edit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(await screen.findByText("Invalid quantity")).toBeInTheDocument();
        expect(mocks.navigate).not.toHaveBeenCalled();
        fireEvent.change(screen.getByLabelText("Mennyiség"), {
            target: { value: "4" },
        });
        expect(screen.queryByText("Invalid quantity")).not.toBeInTheDocument();
    });

    it("loads an existing reservation and saves via create with its id", async () => {
        mocks.params = { id: "01234567-89ab-cdef-0123-456789abcdef" };
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Mennyiség")).toHaveValue("3,00"),
        );
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() => expect(mocks.navigate).toHaveBeenCalledWith(-1));
        expect(mocks.get).toHaveBeenCalledWith({ uuid: mocks.params.id });
        expect(mocks.create).toHaveBeenCalledWith({
            input: expect.objectContaining({
                id: mocks.params.id,
                inventoryId: "old",
                quantity: "3",
                referenceType: "worksheets",
                referenceId: "ref",
            }),
        });
    });

    it("creates inventory in the embedded dialog and refreshes the choices", async () => {
        const { container } = render(<Edit />);
        await waitFor(() =>
            expect(mocks.list).toHaveBeenCalledWith({ list: "inventory" }),
        );
        fireEvent.click(
            container.querySelector("button:has(svg.lucide-plus)")!,
        );
        const dialog = screen.getByRole("dialog", {
            name: "Raktárkészlet létrehozása",
        });
        fireEvent.click(
            within(dialog).getByRole("button", { name: "Create inventory" }),
        );
        await waitFor(() =>
            expect(screen.queryByRole("dialog")).not.toBeInTheDocument(),
        );
        expect(mocks.list).toHaveBeenCalledTimes(2);
        await waitFor(() =>
            expect(screen.getAllByText("New inventory").length).toBeGreaterThan(
                0,
            ),
        );
    });
});
