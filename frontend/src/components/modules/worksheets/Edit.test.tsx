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
    setListResponse: vi.fn(),
}));

vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: "worksheet-id" }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_select_list.ts", () => ({
    useSelectList: () => ({ setListResponse: mocks.setListResponse }),
}));
vi.mock("@/components/modules/worksheets/lib/slice.ts", () => ({
    get: (payload: unknown) => ({ kind: "get", payload }),
    select_list: (payload: unknown) => ({ kind: "list", payload }),
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

describe("worksheet customer dialog", () => {
    beforeEach(() => {
        mocks.dispatch.mockReset();
        mocks.navigate.mockReset();
        mocks.dispatch.mockImplementation((action: { kind: string }) => ({
            unwrap: async () => ({
                kind: "success",
                data: {
                    data:
                        action.kind === "get"
                            ? {
                                  description: "Original",
                                  customer_id: "old-customer",
                                  status: "open",
                              }
                            : action.kind === "list"
                              ? []
                              : null,
                },
            }),
        }));
    });
    afterEach(cleanup);

    it("selects a newly created customer without leaving the worksheet", async () => {
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Leírás")).toHaveValue("Original"),
        );
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Kept" },
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
                ([action]) => action.kind === "list",
            ),
        ).toHaveLength(2);
        expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
        expect(mocks.navigate).not.toHaveBeenCalled();
        await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, 0));
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenLastCalledWith(
                expect.objectContaining({
                    kind: "update",
                    payload: expect.objectContaining({
                        input: expect.objectContaining({
                            customer_id: "new-customer",
                        }),
                    }),
                }),
            ),
        );
    });

    it("cancels the dialog without losing worksheet changes", async () => {
        render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Leírás")).toHaveValue("Original"),
        );
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Kept" },
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
        expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});
