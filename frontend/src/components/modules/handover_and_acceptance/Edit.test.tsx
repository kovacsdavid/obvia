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
    timezone: "Europe/Budapest" as string | undefined,
    dispatch: vi.fn(),
}));

vi.mock("react-router", () => ({
    useNavigate: () => vi.fn(),
    useParams: () => ({ id: mocks.id }),
}));
vi.mock("@/hooks/use_auth", () => ({
    useAuth: () => ({ claims: { tz: mocks.timezone } }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/modules/handover_and_acceptance/lib/slice.ts", () => ({
    select_list: (payload: unknown) => ({ kind: "list", payload }),
    get_full: (payload: unknown) => ({ kind: "get", payload }),
    create: (payload: unknown) => ({ kind: "create", payload }),
    update: (payload: unknown) => ({ kind: "update", payload }),
}));
vi.mock("@/components/modules/customers/Edit.tsx", () => ({
    default: ({
        createOnly,
        onSuccess,
        onCancel,
    }: {
        createOnly?: boolean;
        onSuccess: (customer: { id: string }) => void;
        onCancel: () => void;
    }) => (
        <div>
            <span>
                {createOnly ? "Customer create-only" : "Customer route mode"}
            </span>
            <button
                type="button"
                onClick={() => onSuccess({ id: "new-customer" })}
            >
                Complete customer
            </button>
            <button type="button" onClick={onCancel}>
                Cancel customer
            </button>
        </div>
    ),
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: vi.fn() } }));

describe("receipt date and time form", () => {
    beforeEach(() => {
        mocks.id = undefined;
        mocks.timezone = "Europe/Budapest";
        mocks.dispatch.mockReset();
        mocks.dispatch.mockImplementation((action: { kind: string }) => ({
            unwrap: async () => ({
                kind: "success",
                data: {
                    data:
                        action.kind === "get"
                            ? {
                                  customer: { id: "customer" },
                                  item_identifier_type: { id: "identifier" },
                                  item_identifier_value: "serial",
                                  received_at: "2026-10-25T01:30:12.123456Z",
                              }
                            : action.kind === "list"
                              ? []
                              : null,
                },
            }),
        }));
    });
    afterEach(() => {
        cleanup();
        vi.useRealTimers();
    });

    it("defaults to now in the account timezone and submits that instant untouched", async () => {
        vi.useFakeTimers({ toFake: ["Date"] });
        vi.setSystemTime(new Date("2026-07-02T10:30:45.123Z"));
        render(<Edit />);
        const field = screen.getByLabelText(/Átvétel dátuma és időpontja/);
        await waitFor(() =>
            expect(field).toHaveValue("2026-07-02T12:30"),
        );

        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(
                expect.objectContaining({
                    kind: "create",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            receivedAt: "2026-07-02T10:30:45.123Z",
                        }),
                    }),
                }),
            ),
        );
    });

    it("submits the exact default instant during a repeated hour", async () => {
        vi.useFakeTimers({ toFake: ["Date"] });
        vi.setSystemTime(new Date("2026-10-25T00:30:45.123Z"));
        render(<Edit />);
        const field = screen.getByLabelText(/Átvétel dátuma és időpontja/);
        await waitFor(() =>
            expect(field).toHaveValue("2026-10-25T02:30"),
        );
        expect(screen.queryByText("Időeltolás")).not.toBeInTheDocument();

        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(
                expect.objectContaining({
                    kind: "create",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            receivedAt: "2026-10-25T00:30:45.123Z",
                        }),
                    }),
                }),
            ),
        );
    });

    it("sends a new local time as UTC", async () => {
        render(<Edit />);
        fireEvent.change(screen.getByLabelText(/Átvétel dátuma és időpontja/), {
            target: { value: "2026-07-02T12:30" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(
                expect.objectContaining({
                    kind: "create",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            receivedAt: "2026-07-02T10:30:00.000Z",
                        }),
                    }),
                }),
            ),
        );
    });

    it("preserves an untouched timestamp in the repeated hour including fractional seconds", async () => {
        mocks.id = "existing";
        render(<Edit />);
        const field = screen.getByLabelText(/Átvétel dátuma és időpontja/);
        await waitFor(() =>
            expect(field).toHaveValue("2026-10-25T02:30"),
        );
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(
                expect.objectContaining({
                    kind: "update",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            receivedAt: "2026-10-25T01:30:12.123456Z",
                        }),
                    }),
                }),
            ),
        );
    });

    it("blocks nonexistent times and repeated times without an offset choice", async () => {
        render(<Edit />);
        const field = screen.getByLabelText(/Átvétel dátuma és időpontja/);
        fireEvent.change(field, { target: { value: "2026-03-29T02:30" } });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(
            screen.getByText(/óraátállítás miatt nem létezik/),
        ).toBeInTheDocument();
        fireEvent.change(field, { target: { value: "2026-10-25T02:30" } });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(
            screen.getByText(/Válassza ki az időeltolást/),
        ).toBeInTheDocument();
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "create" }),
        );
    });

    it("requires a valid account timezone", () => {
        mocks.timezone = undefined;
        render(<Edit />);
        fireEvent.change(screen.getByLabelText(/Átvétel dátuma és időpontja/), {
            target: { value: "2026-07-02T12:30" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(
            screen.getByText(/időzóna nincs megadva vagy érvénytelen/),
        ).toBeInTheDocument();
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "create" }),
        );
    });

    it("submits the chosen offset for a repeated hour", async () => {
        render(<Edit />);
        fireEvent.change(screen.getByLabelText(/Átvétel dátuma és időpontja/), {
            target: { value: "2026-10-25T02:30" },
        });
        fireEvent.keyDown(
            screen.getByRole("combobox", { name: "Időeltolás" }),
            { key: "ArrowDown" },
        );
        fireEvent.click(
            await screen.findByRole("option", { name: "UTC+01:00" }),
        );
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(
                expect.objectContaining({
                    kind: "create",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            receivedAt: "2026-10-25T01:30:00.000Z",
                        }),
                    }),
                }),
            ),
        );
    });

    it("creates a customer in an edit dialog and selects it on the handover", async () => {
        mocks.id = "handover-id";
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Azonosító értéke")).toHaveValue(
                "serial",
            ),
        );
        fireEvent.change(screen.getByLabelText("Azonosító értéke"), {
            target: { value: "kept" },
        });
        fireEvent.click(screen.getAllByRole("button")[0]);
        expect(screen.getByText("Customer create-only")).toBeInTheDocument();
        fireEvent.click(
            screen.getByRole("button", { name: "Complete customer" }),
        );
        await waitFor(() =>
            expect(
                screen.queryByText("Customer create-only"),
            ).not.toBeInTheDocument(),
        );
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) =>
                    action.kind === "list" &&
                    action.payload.list === "customers",
            ),
        ).toHaveLength(2);
        expect(screen.getByLabelText("Azonosító értéke")).toHaveValue("kept");
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(
                expect.objectContaining({
                    kind: "update",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            customerId: "new-customer",
                        }),
                    }),
                }),
            ),
        );
    });

    it("closes the customer dialog without changing the handover", async () => {
        mocks.id = "handover-id";
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Azonosító értéke")).toHaveValue(
                "serial",
            ),
        );
        fireEvent.change(screen.getByLabelText("Azonosító értéke"), {
            target: { value: "kept" },
        });
        fireEvent.click(screen.getAllByRole("button")[0]);
        fireEvent.click(
            screen.getByRole("button", { name: "Cancel customer" }),
        );
        await waitFor(() =>
            expect(
                screen.queryByText("Customer create-only"),
            ).not.toBeInTheDocument(),
        );
        expect(screen.getByLabelText("Azonosító értéke")).toHaveValue("kept");
    });
});
