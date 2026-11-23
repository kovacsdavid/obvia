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

import { cleanup, render, screen } from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { MemoryRouter, Route, Routes } from "react-router";
import { afterEach, describe, expect, it, vi } from "vitest";
import View from "./View";

const mocks = vi.hoisted(() => ({ dispatch: vi.fn(), get: vi.fn() }));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/inventory_reservations/lib/slice.ts", () => ({
    get_resolved: (payload: unknown) => ({ kind: "get", payload }),
}));
vi.mock("@/components/modules/activity_feed/ActivityFeed", () => ({
    default: ({ resourceId }: { resourceId: string }) => (
        <div>Feed: {resourceId}</div>
    ),
}));
afterEach(cleanup);

describe("reservation details", () => {
    it("loads the resolved reservation and retains the activity feed", async () => {
        mocks.get.mockResolvedValue({
            kind: "success",
            data: {
                data: {
                    id: "res-1",
                    inventory_id: "inv-1",
                    quantity: "4",
                    reference_type: null,
                    reference_id: null,
                    reserved_until: null,
                    status: "active",
                    created_by: "Anna",
                    created_at: "2026-01-01",
                    updated_at: "2026-01-01",
                },
            },
        });
        mocks.dispatch.mockImplementation((action: { payload: unknown }) => ({
            unwrap: () => mocks.get(action.payload),
        }));
        render(
            <MemoryRouter initialEntries={["/reservation/res-1"]}>
                <Routes>
                    <Route path="/reservation/:id" element={<View />} />
                </Routes>
            </MemoryRouter>,
        );
        expect(await screen.findByText("Anna")).toBeInTheDocument();
        expect(screen.getByText("Feed: res-1")).toBeInTheDocument();
        expect(mocks.get).toHaveBeenCalledWith({ uuid: "res-1" });
    });
});
