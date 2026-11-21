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
import TaxesEdit from "./Edit";
import ServicesEdit from "../services/Edit";
import TasksEdit from "../tasks/Edit";

const mocks = vi.hoisted(() => {
    const action = (kind: string) =>
        Object.assign((payload: unknown) => ({ kind, payload }), {
            fulfilled: {
                match: (response: { kind: string }) => response.kind === kind,
            },
        });
    return {
        id: undefined as string | undefined,
        dispatch: vi.fn(),
        navigate: vi.fn(),
        logout: vi.fn(),
        taxes: {
            create: action("tax/create"),
            get: action("tax/get"),
            update: action("tax/update"),
            select_list: action("tax/list"),
        },
        services: {
            create: action("service/create"),
            get: action("service/get"),
            update: action("service/update"),
            select_list: action("service/list"),
        },
        tasks: {
            create: action("task/create"),
            get: action("task/get"),
            update: action("task/update"),
            select_list: action("task/list"),
        },
    };
});

vi.mock("react-router", () => ({
    useNavigate: () => mocks.navigate,
    useParams: () => ({ id: mocks.id }),
}));
vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({
    useAuth: () => ({ logout: mocks.logout }),
}));
vi.mock("@/components/modules/taxes/lib/slice.ts", () => mocks.taxes);
vi.mock("@/components/modules/services/lib/slice.ts", () => mocks.services);
vi.mock("@/components/modules/tasks/lib/slice.ts", () => mocks.tasks);
vi.mock("@/components/modules/worksheets/Edit.tsx", () => ({
    default: () => null,
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: vi.fn() } }));

const tax = {
    id: "new-tax",
    rate: "27",
    description: "Existing tax",
    country_code: "HU",
    tax_category: "standard",
    is_rate_applicable: true,
    legal_text: null,
    reporting_code: null,
    is_default: false,
    status: "active",
};

beforeEach(() => {
    mocks.id = undefined;
    mocks.dispatch.mockReset();
    mocks.navigate.mockReset();
    mocks.dispatch.mockImplementation((action: { kind: string }) => {
        const data =
            action.kind === "tax/get" || action.kind === "tax/create"
                ? tax
                : action.kind === "service/get"
                  ? {
                        name: "Service",
                        description: "Original",
                        default_price: "",
                        default_tax_id: "old-tax",
                        currency_code: "HUF",
                        status: "active",
                        unit_of_measure_id: "unit",
                    }
                  : action.kind === "task/get"
                    ? {
                          worksheet_id: "worksheet",
                          service_id: "service",
                          currency_code: "HUF",
                          description: "Original",
                          quantity: "1",
                          price: "1",
                          tax_id: "old-tax",
                          status: "in_progress",
                          priority: "normal",
                          due_date: null,
                      }
                    : [];
        if (action.kind.startsWith("task/")) {
            return {
                unwrap: async () => ({ kind: "success", data: { data } }),
            };
        }
        return Promise.resolve({
            kind: action.kind,
            payload: {
                statusCode: action.kind.endsWith("/create") ? 201 : 200,
                jsonData: { data },
            },
        });
    });
});
afterEach(cleanup);

describe("tax editor mode", () => {
    it("creates independently of the enclosing route", async () => {
        mocks.id = "service-id";
        const onSuccess = vi.fn();
        render(<TaxesEdit createOnly onSuccess={onSuccess} />);
        fireEvent.change(screen.getByLabelText("Megnevezés"), {
            target: { value: "New tax" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(tax));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "tax/create",
            payload: expect.objectContaining({
                id: null,
                description: "New tax",
            }),
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "tax/get" }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "tax/update" }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("preserves standalone fetch and update behavior", async () => {
        mocks.id = "tax-id";
        render(<TaxesEdit />);
        await waitFor(() =>
            expect(screen.getByLabelText("Megnevezés")).toHaveValue(
                "Existing tax",
            ),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "tax/get",
            payload: "tax-id",
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/ado/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "tax/update",
            payload: expect.objectContaining({ id: "tax-id" }),
        });
    });

    it("continues to create on the standalone creation route", async () => {
        render(<TaxesEdit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/ado/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "tax/create",
            payload: expect.objectContaining({ id: null }),
        });
    });
});

describe.each([
    {
        label: "service",
        Component: ServicesEdit,
        buttonIndex: 0,
        field: "defaultTaxId",
    },
    { label: "task", Component: TasksEdit, buttonIndex: 2, field: "taxId" },
])("$label tax dialog", ({ label, Component, buttonIndex, field }) => {
    it("creates and selects a tax without leaving the parent", async () => {
        mocks.id = `${label}-id`;
        const { container } = render(<Component />);
        await waitFor(() =>
            expect(screen.getByLabelText("Leírás")).toHaveValue("Original"),
        );
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Kept" },
        });
        fireEvent.click(
            container.querySelectorAll("button:has(svg.lucide-plus)")[
                buttonIndex
            ],
        );
        const modal = screen.getByRole("dialog", { name: "Adó létrehozása" });
        fireEvent.change(within(modal).getByLabelText("Megnevezés"), {
            target: { value: "Created tax" },
        });
        fireEvent.click(
            within(modal).getByRole("button", { name: "Létrehozás" }),
        );
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", { name: "Adó létrehozása" }),
            ).not.toBeInTheDocument(),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "tax/create",
            payload: expect.objectContaining({ id: null }),
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "tax/get" }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "tax/update" }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
        expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) =>
                    action.kind === `${label}/list` &&
                    (action.payload === "taxes" ||
                        action.payload.list === "taxes"),
            ),
        ).toHaveLength(2);
        await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, 0));
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        const input = expect.objectContaining({ [field]: "new-tax" });
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: `${label}/update`,
                payload: label === "task" ? { input } : input,
            }),
        );
    });

    it("cancels without losing parent changes", async () => {
        mocks.id = `${label}-id`;
        const { container } = render(<Component />);
        await waitFor(() =>
            expect(screen.getByLabelText("Leírás")).toHaveValue("Original"),
        );
        fireEvent.change(screen.getByLabelText("Leírás"), {
            target: { value: "Kept" },
        });
        fireEvent.click(
            container.querySelectorAll("button:has(svg.lucide-plus)")[
                buttonIndex
            ],
        );
        const modal = screen.getByRole("dialog", { name: "Adó létrehozása" });
        fireEvent.click(within(modal).getByRole("button", { name: "Mégse" }));
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", { name: "Adó létrehozása" }),
            ).not.toBeInTheDocument(),
        );
        expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
        expect(mocks.navigate).not.toHaveBeenCalled();
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "tax/create" }),
        );
    });
});
