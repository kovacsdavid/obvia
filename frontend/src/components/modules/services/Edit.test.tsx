import {
    act,
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
vi.mock("@/components/modules/services/lib/slice.ts", () => ({
    create: (payload: unknown) => ({ kind: "create", payload }),
    get: (payload: unknown) => ({ kind: "get", payload }),
    update: (payload: unknown) => ({ kind: "update", payload }),
    select_list: (payload: unknown) => ({ kind: "list", payload }),
}));
vi.mock("@/components/modules/taxes/Edit.tsx", () => ({
    default: ({
        createOnly,
        showCard,
        onSuccess,
        onCancel,
    }: {
        createOnly: boolean;
        showCard: boolean;
        onSuccess: (tax: { id: string }) => void;
        onCancel: () => void;
    }) => (
        <div>
            <span>
                {createOnly && !showCard ? "Tax create-only" : "Tax route mode"}
            </span>
            <button type="button" onClick={() => onSuccess({ id: "new-tax" })}>
                Complete tax
            </button>
            <button type="button" onClick={onCancel}>
                Cancel tax
            </button>
        </div>
    ),
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: mocks.toast } }));

const service = {
    id: "service-id",
    name: "Existing service",
    description: "Original description",
    default_price: "1250.5",
    default_tax_id: "tax-id",
    currency_code: "EUR",
    status: "inactive",
    unit_of_measure_id: "unit-id",
    created_by_id: "user-id",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
    deleted_at: null,
};

const lists: Record<string, { value: string; title: string }[]> = {
    currencies: [
        { value: "EUR", title: "Euro" },
        { value: "HUF", title: "Forint" },
    ],
    taxes: [
        { value: "tax-id", title: "Standard tax" },
        { value: "new-tax", title: "New tax" },
    ],
    units_of_measure: [{ value: "unit-id", title: "Hour" }],
};

type Action = { kind: string; payload: { list?: string } };

describe("service editor", () => {
    beforeEach(() => {
        mocks.id = undefined;
        mocks.dispatch.mockReset();
        mocks.navigate.mockReset();
        mocks.toast.mockReset();
        mocks.dispatch.mockImplementation((action: Action) => ({
            unwrap: async () => ({
                kind: "success",
                data: {
                    data:
                        action.kind === "list"
                            ? lists[action.payload.list ?? ""]
                            : service,
                },
            }),
        }));
    });
    afterEach(cleanup);

    it("creates with object arguments and calls the success callback in create-only mode", async () => {
        mocks.id = "parent-id";
        const onSuccess = vi.fn();
        render(<Edit createOnly onSuccess={onSuccess} />);

        fireEvent.change(screen.getByLabelText("Megnevezés"), {
            target: { value: "New service" },
        });
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Details" },
        });
        fireEvent.change(screen.getByLabelText("Alapértelmezett ár"), {
            target: { value: "1250.5" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));

        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(service));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "create",
            payload: {
                input: {
                    id: null,
                    name: "New service",
                    description: "Details",
                    defaultPrice: "1250.5",
                    defaultTaxId: "",
                    currencyCode: "HUF",
                    status: "active",
                    unitOfMeasureId: "",
                    newUnitOfMeasure: "",
                },
            },
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "get" }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "update" }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
        expect(mocks.toast).toHaveBeenCalledWith(
            expect.objectContaining({ type: "success" }),
        );
    });

    it("navigates after a standalone create", async () => {
        render(<Edit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/szolgaltatas/lista"),
        );
    });

    it("waits for all lists before fetching the existing service", async () => {
        mocks.id = "service-id";
        let finishTaxList!: (value: unknown) => void;
        const taxList = new Promise((resolve) => {
            finishTaxList = resolve;
        });
        const originalDispatch = mocks.dispatch.getMockImplementation()!;
        mocks.dispatch.mockImplementation((action: Action) =>
            action.kind === "list" && action.payload.list === "taxes"
                ? { unwrap: () => taxList }
                : originalDispatch(action),
        );
        render(<Edit />);
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "get" }),
        );
        await act(async () => {
            finishTaxList({ kind: "success", data: { data: lists.taxes } });
        });
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "get",
                payload: { uuid: "service-id" },
            }),
        );
        expect(screen.getByLabelText("Megnevezés")).toHaveValue(
            "Existing service",
        );
    });

    it("loads lists and the existing service, then updates with populated values", async () => {
        mocks.id = "service-id";
        render(<Edit />);

        await waitFor(() =>
            expect(screen.getByLabelText("Megnevezés")).toHaveValue(
                "Existing service",
            ),
        );
        for (const list of ["currencies", "taxes", "units_of_measure"]) {
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "list",
                payload: { list },
            });
        }
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "get",
            payload: { uuid: "service-id" },
        });
        expect(
            screen.getByText("Euro", {
                selector: "span[data-slot='select-value']",
            }),
        ).toBeVisible();
        expect(
            screen.getByText("Standard tax", {
                selector: "span[data-slot='select-value']",
            }),
        ).toBeVisible();
        expect(
            screen.getByText("Hour", {
                selector: "span[data-slot='select-value']",
            }),
        ).toBeVisible();
        expect(
            screen.getByText("Inaktív", {
                selector: "span[data-slot='select-value']",
            }),
        ).toBeVisible();
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Changed" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));

        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/szolgaltatas/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "update",
            payload: {
                input: {
                    id: "service-id",
                    name: "Existing service",
                    description: "Changed",
                    defaultPrice: "1250.5",
                    defaultTaxId: "tax-id",
                    currencyCode: "EUR",
                    status: "inactive",
                    unitOfMeasureId: "unit-id",
                    newUnitOfMeasure: "",
                },
            },
        });
        expect(mocks.toast).toHaveBeenCalledWith(
            expect.objectContaining({ type: "success" }),
        );
    });

    it.each(["create", "update"])(
        "shows and independently clears backend field errors on %s",
        async (kind) => {
            if (kind === "update") mocks.id = "service-id";
            const originalDispatch = mocks.dispatch.getMockImplementation()!;
            mocks.dispatch.mockImplementation((action: Action) =>
                action.kind === kind
                    ? {
                          unwrap: async () => ({
                              kind: "http_error",
                              status: 422,
                              body: {
                                  error: {
                                      fields: {
                                          name: "Name is required",
                                          description: "Description is invalid",
                                      },
                                  },
                              },
                          }),
                      }
                    : originalDispatch(action),
            );
            render(<Edit />);
            if (kind === "update")
                await waitFor(() =>
                    expect(screen.getByLabelText("Megnevezés")).toHaveValue(
                        "Existing service",
                    ),
                );
            fireEvent.click(
                screen.getByRole("button", {
                    name: kind === "create" ? "Létrehozás" : "Módosítás",
                }),
            );

            expect(
                await screen.findByText("Name is required"),
            ).toBeInTheDocument();
            expect(
                screen.getByText("Description is invalid"),
            ).toBeInTheDocument();
            expect(screen.getByLabelText("Megnevezés")).toHaveAttribute(
                "aria-invalid",
                "true",
            );
            fireEvent.change(screen.getByLabelText("Megnevezés"), {
                target: { value: "Fixed" },
            });
            expect(
                screen.queryByText("Name is required"),
            ).not.toBeInTheDocument();
            expect(screen.getByLabelText("Megnevezés")).toHaveAttribute(
                "aria-invalid",
                "false",
            );
            expect(
                screen.getByText("Description is invalid"),
            ).toBeInTheDocument();
            fireEvent.change(screen.getByLabelText("Leírás"), {
                target: { value: "Fixed" },
            });
            expect(
                screen.queryByText("Description is invalid"),
            ).not.toBeInTheDocument();
            expect(mocks.navigate).not.toHaveBeenCalled();
            expect(mocks.toast).not.toHaveBeenCalled();
        },
    );

    it("refreshes lists and selects a newly created nested tax", async () => {
        mocks.id = "service-id";
        let taxLoads = 0;
        const originalDispatch = mocks.dispatch.getMockImplementation()!;
        mocks.dispatch.mockImplementation((action: Action) => {
            if (action.kind === "list" && action.payload.list === "taxes") {
                taxLoads += 1;
                return {
                    unwrap: async () => ({
                        kind: "success",
                        data: {
                            data:
                                taxLoads === 1
                                    ? lists.taxes.slice(0, 1)
                                    : lists.taxes,
                        },
                    }),
                };
            }
            return originalDispatch(action);
        });
        const { container } = render(<Edit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Leírás")).toHaveValue(
                "Original description",
            ),
        );
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Kept" },
        });
        fireEvent.click(
            container.querySelector("button:has(svg.lucide-plus)")!,
        );
        const dialog = screen.getByRole("dialog", { name: "Adó létrehozása" });
        expect(within(dialog).getByText("Tax create-only")).toBeInTheDocument();
        fireEvent.click(
            within(dialog).getByRole("button", { name: "Complete tax" }),
        );
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", { name: "Adó létrehozása" }),
            ).not.toBeInTheDocument(),
        );
        await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, 0));
        });
        expect(
            screen.getByText("New tax", {
                selector: "span[data-slot='select-value']",
            }),
        ).toBeVisible();
        expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
        expect(mocks.navigate).not.toHaveBeenCalled();
        for (const list of ["currencies", "taxes", "units_of_measure"]) {
            expect(
                mocks.dispatch.mock.calls.filter(
                    ([action]) =>
                        action.kind === "list" && action.payload.list === list,
                ),
            ).toHaveLength(2);
        }
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "update",
                payload: {
                    input: expect.objectContaining({
                        defaultTaxId: "new-tax",
                        description: "Kept",
                    }),
                },
            }),
        );
    });
});
