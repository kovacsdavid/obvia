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
}));

vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: mocks.id }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/modules/customers/lib/slice.ts", () => ({
    get_full: (payload: unknown) => ({ kind: "get", payload }),
    create: (payload: unknown) => ({ kind: "create", payload }),
    update: (payload: unknown) => ({ kind: "update", payload }),
}));
vi.mock("@/components/modules/address/Address", () => ({
    default: () => null,
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: vi.fn() } }));

const customer = {
    id: "customer-id",
    name: "Customer",
    contact_name: null,
    email: null,
    phone_number: null,
    status: "active",
    customer_type: "natural",
    billing_address: null,
    mailing_address: null,
};

describe("customer editor mode", () => {
    beforeEach(() => {
        mocks.id = undefined;
        mocks.dispatch.mockReset();
        mocks.navigate.mockReset();
        mocks.dispatch.mockImplementation(() => ({
            unwrap: async () => ({ kind: "success", data: { data: customer } }),
        }));
    });
    afterEach(cleanup);

    it("creates independently of the enclosing route and calls success", async () => {
        mocks.id = "handover-id";
        const onSuccess = vi.fn();
        render(<Edit createOnly onSuccess={onSuccess} />);

        expect(screen.getByText("Vevő létrehozás")).toBeInTheDocument();
        expect(mocks.dispatch).not.toHaveBeenCalled();
        fireEvent.change(screen.getByLabelText("Név"), {
            target: { value: "New customer" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));

        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(customer));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: {
                input: expect.objectContaining({
                    id: null,
                    name: "New customer",
                }),
            },
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "get" }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "update" }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("calls cancel without navigating in create-only mode", () => {
        mocks.id = "worksheet-id";
        const onCancel = vi.fn();
        render(<Edit createOnly onCancel={onCancel} />);

        fireEvent.click(screen.getByRole("button", { name: "Mégse" }));
        expect(onCancel).toHaveBeenCalledOnce();
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("continues to fetch and update a standalone customer", async () => {
        mocks.id = "customer-id";
        render(<Edit />);

        await waitFor(() =>
            expect(screen.getByLabelText("Név")).toHaveValue("Customer"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "get",
            payload: { uuid: "customer-id" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/vevo/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "update",
            payload: { input: expect.objectContaining({ id: "customer-id" }) },
        });
    });

    it("continues to create on the standalone creation route", async () => {
        render(<Edit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));

        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/vevo/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: { input: expect.objectContaining({ id: null }) },
        });
    });
});
