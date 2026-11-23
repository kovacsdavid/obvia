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
import { afterEach, expect, it, vi } from "vitest";
import View from "./View";

const mocks = vi.hoisted(() => ({ dispatch: vi.fn(), navigate: vi.fn() }));
vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: "warehouse-id" }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/activity_feed/ActivityFeed", () => ({
    default: () => null,
}));
vi.mock("@/components/modules/warehouses/lib/slice.ts", () => ({
    get_resolved: (payload: unknown) => ({ kind: "get_resolved", payload }),
}));
afterEach(cleanup);

it("loads the resolved warehouse and renders it", async () => {
    mocks.dispatch.mockReturnValue({
        unwrap: async () => ({
            kind: "success",
            data: {
                data: {
                    id: "warehouse-id",
                    name: "Main warehouse",
                    contact_name: "Alice",
                    contact_phone: null,
                    status: "active",
                    created_by: "Anna",
                    created_at: "2026-01-01T00:00:00Z",
                    updated_at: "2026-01-01T00:00:00Z",
                    deleted_at: null,
                },
            },
        }),
    });
    render(<View />);
    expect(await screen.findByText("Main warehouse")).toBeInTheDocument();
    expect(screen.getByText("Alice")).toBeInTheDocument();
    expect(mocks.dispatch).toHaveBeenCalledWith({
        kind: "get_resolved",
        payload: { uuid: "warehouse-id" },
    });
});
