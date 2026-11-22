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
import List from "./List";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    orderSelect: vi.fn(),
    filterSelect: vi.fn(),
    setPage: vi.fn(),
    setLimit: vi.fn(),
    setTotal: vi.fn(),
    paginatorSelect: vi.fn(),
}));

vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/hooks/use_data_display_common.ts", async () => {
    const { useState } = await import("react");
    return {
        useDataDisplayCommon: function useDataDisplayCommon() {
            const [filterValue, setFilterValue] = useState("");
            return {
                rawQuery: "",
                page: 1,
                totalPages: 1,
                orderBy: "created_at",
                order: "asc",
                filterValue,
                setFilterValue,
                ...mocks,
            };
        },
    };
});
vi.mock("@/components/modules/handover_and_acceptance/lib/slice.ts", () => ({
    list: (payload: unknown) => ({ kind: "list", payload }),
    deleteItem: vi.fn(),
    print: vi.fn(),
}));

async function renderList() {
    render(
        <MemoryRouter>
            <List />
        </MemoryRouter>,
    );
    await waitFor(() => expect(mocks.setTotal).toHaveBeenCalledWith(0));
}

describe("handover list query controls", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.dispatch.mockReturnValue({
            unwrap: async () => ({
                kind: "success",
                data: {
                    meta: { page: 1, limit: 25, total: 0 },
                    data: [],
                },
            }),
        });
    });
    afterEach(cleanup);

    it.each([
        [1, "display_id"],
        [2, "customer"],
        [3, "item_make"],
        [4, "item_model"],
        [6, "received_at"],
    ])("renders column %i (%s) without sorting", async (index) => {
        await renderList();
        const header = screen.getAllByRole("columnheader")[index];

        expect(header.style.cursor).not.toBe("pointer");
        expect(header.style.textDecoration).not.toBe("underline");
        expect(header.querySelector("svg")).toBeNull();
        fireEvent.click(header);
        expect(mocks.orderSelect).not.toHaveBeenCalled();
    });

    it.each([
        [7, "created_at"],
        [8, "updated_at"],
    ])("keeps column %i (%s) sortable", async (index, field) => {
        await renderList();
        const header = screen.getAllByRole("columnheader")[index];

        expect(header).toHaveStyle({
            cursor: "pointer",
            textDecoration: "underline",
        });
        fireEvent.click(header);
        expect(mocks.orderSelect).toHaveBeenCalledExactlyOnceWith(field);
    });

    it("filters by the complete document ID on blur and allows clearing", async () => {
        await renderList();
        fireEvent.click(screen.getByRole("button", { name: /Szűrő/ }));
        const input = await screen.findByRole("textbox", { name: "Azonosító" });

        fireEvent.change(input, { target: { value: "DEMO-AT-2026-1" } });
        expect(input).toHaveValue("DEMO-AT-2026-1");
        expect(mocks.filterSelect).not.toHaveBeenCalled();
        fireEvent.blur(input);
        expect(mocks.filterSelect).toHaveBeenCalledExactlyOnceWith(
            "display_id",
            "DEMO-AT-2026-1",
        );

        fireEvent.change(input, { target: { value: "" } });
        fireEvent.blur(input);
        expect(mocks.filterSelect).toHaveBeenLastCalledWith("display_id", "");
    });
});
