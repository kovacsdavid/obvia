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
import Edit from "./Edit";
import List from "./List";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    responses: {} as Record<string, unknown>,
}));
vi.mock("@/store/hooks.ts", () => ({
    useAppDispatch: () => mocks.dispatch,
    useAppSelector: () => null,
}));
vi.mock("@/hooks/use_auth", () => ({
    useAuth: () => ({ claims: null, logout: vi.fn() }),
}));
vi.mock("@/components/modules/auth/lib/slice.ts", () => ({
    updateToken: (payload: unknown) => ({ kind: "updateToken", payload }),
}));
vi.mock("@/components/modules/databases/lib/slice.ts", () => {
    const action = (kind: string) =>
        Object.assign((payload: unknown) => ({ kind, payload }), {
            fulfilled: {
                match: (value: { type: string }) =>
                    value.type === `${kind}/fulfilled`,
            },
        });
    return {
        create: action("create"),
        activate: action("activate"),
        list: action("list"),
        get: action("get"),
        deleteDatabase: action("delete"),
    };
});

const database = {
    id: "tenant",
    name: "Company",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
};
const tokenData = { token: "fresh", claims: { active_tenant: "tenant" } };
const success = (data: unknown, status = 200) => ({
    kind: "success",
    status,
    data: { data },
});

describe("database UI flows", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.responses = {
            create: success(database, 201),
            activate: success(tokenData),
            delete: success(tokenData),
            list: {
                kind: "success",
                status: 200,
                data: {
                    data: [database],
                    meta: { page: 1, limit: 25, total: 1 },
                },
            },
        };
        mocks.dispatch.mockImplementation(
            (action: { kind: string; payload: unknown }) => {
                if (action.kind === "updateToken") return action;
                return Promise.resolve({
                    type: `${action.kind}/fulfilled`,
                    payload: mocks.responses[action.kind],
                });
            },
        );
    });
    afterEach(cleanup);

    it("activates a created database before updating the token and calling success", async () => {
        const onSuccess = vi.fn();
        render(
            <MemoryRouter>
                <Edit onSuccess={onSuccess} />
            </MemoryRouter>,
        );
        fireEvent.change(screen.getByLabelText("Adatbázis neve"), {
            target: { value: "Company" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(database));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: { input: { name: "Company" } },
        });
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "activate",
            payload: { uuid: "tenant" },
        });
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "updateToken",
            payload: tokenData,
        });
    });

    it("shows backend field errors without activating", async () => {
        mocks.responses.create = {
            kind: "http_error",
            status: 422,
            body: {
                error: { message: "Invalid", fields: { name: "Required" } },
            },
        };
        render(
            <MemoryRouter>
                <Edit />
            </MemoryRouter>,
        );
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        expect(await screen.findByText("Required")).toBeInTheDocument();
        expect(screen.getByLabelText("Adatbázis neve")).toHaveAttribute(
            "aria-invalid",
            "true",
        );
        expect(screen.queryByText("Invalid")).not.toBeInTheDocument();
        fireEvent.change(screen.getByLabelText("Adatbázis neve"), {
            target: { value: "Company" },
        });
        expect(screen.queryByText("Required")).not.toBeInTheDocument();
        expect(screen.getByLabelText("Adatbázis neve")).toHaveAttribute(
            "aria-invalid",
            "false",
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "activate" }),
        );
    });

    it("does not update the token or finish creation when activation fails", async () => {
        mocks.responses.activate = { kind: "http_error", status: 401 };
        const onSuccess = vi.fn();
        render(
            <MemoryRouter>
                <Edit onSuccess={onSuccess} />
            </MemoryRouter>,
        );
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "activate",
                payload: { uuid: "tenant" },
            }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "updateToken" }),
        );
        expect(onSuccess).not.toHaveBeenCalled();
    });

    it("replaces the token and refreshes the list after deletion", async () => {
        render(
            <MemoryRouter>
                <List />
            </MemoryRouter>,
        );
        await screen.findByText("Company");
        fireEvent.pointerDown(
            screen.getByRole("button", { name: "Menü megnyitása" }),
            { button: 0, ctrlKey: false },
        );
        fireEvent.click(
            await screen.findByRole("menuitem", { name: "Törlés" }),
        );
        fireEvent.click(await screen.findByRole("button", { name: "Törlés" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "updateToken",
                payload: tokenData,
            }),
        );
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) => action.kind === "list",
            ),
        ).toHaveLength(2);
    });
});
