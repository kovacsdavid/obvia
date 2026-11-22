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
import TasksEdit from "./Edit";
import WorksheetsEdit from "../worksheets/Edit";
import ServicesEdit from "../services/Edit";

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
        tasks: {
            create: action("task/create"),
            get: action("task/get"),
            update: action("task/update"),
            select_list: action("task/list"),
        },
        worksheets: {
            create: action("worksheet/create"),
            get: action("worksheet/get"),
            update: action("worksheet/update"),
            select_list: action("worksheet/list"),
        },
        services: {
            create: action("service/create"),
            get: action("service/get"),
            update: action("service/update"),
            select_list: action("service/list"),
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
vi.mock("@/components/modules/tasks/lib/slice.ts", () => mocks.tasks);
vi.mock("@/components/modules/worksheets/lib/slice.ts", () => mocks.worksheets);
vi.mock("@/components/modules/services/lib/slice.ts", () => mocks.services);
vi.mock("@/components/modules/customers/Edit.tsx", () => ({
    default: () => null,
}));
vi.mock("@/components/modules/taxes/Edit.tsx", () => ({ default: () => null }));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: vi.fn() } }));

const worksheet = {
    id: "new-worksheet",
    description: "Existing worksheet",
    customer_id: "customer",
    status: "open",
};
const service = {
    id: "new-service",
    name: "Existing service",
    description: "",
    default_price: "",
    default_tax_id: "tax",
    currency_code: "HUF",
    status: "active",
    unit_of_measure_id: "unit",
};
const task = {
    worksheet_id: "old-worksheet",
    service_id: "old-service",
    currency_code: "HUF",
    description: "Original task",
    quantity: "1",
    price: "1",
    tax_id: "tax",
    status: "in_progress",
    priority: "normal",
    due_date: null,
};

beforeEach(() => {
    mocks.id = undefined;
    mocks.dispatch.mockReset();
    mocks.navigate.mockReset();
    mocks.dispatch.mockImplementation((action: { kind: string }) => {
        const data =
            action.kind === "task/get"
                ? task
                : action.kind === "worksheet/get" ||
                    action.kind === "worksheet/create"
                  ? worksheet
                  : action.kind === "service/get" ||
                      action.kind === "service/create"
                    ? service
                    : [];
        if (!action.kind.startsWith("service/")) {
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

describe.each([
    {
        label: "worksheet",
        Component: WorksheetsEdit,
        dialog: "Munkalap létrehozása",
        nameField: "Leírás",
        initial: "Existing worksheet",
        path: "/munkalap/lista",
        field: "worksheetId",
        index: 0,
        record: worksheet,
    },
    {
        label: "service",
        Component: ServicesEdit,
        dialog: "Szolgáltatás létrehozása",
        nameField: "Megnevezés",
        initial: "Existing service",
        path: "/szolgaltatas/lista",
        field: "serviceId",
        index: 1,
        record: service,
    },
])(
    "$label creation mode",
    ({
        label,
        Component,
        dialog,
        nameField,
        initial,
        path,
        field,
        index,
        record,
    }) => {
        it("creates in the task dialog and selects the new record", async () => {
            mocks.id = "task-id";
            const { container } = render(<TasksEdit />);
            await waitFor(() =>
                expect(screen.getByLabelText("Leírás")).toHaveValue(
                    "Original task",
                ),
            );
            fireEvent.change(screen.getByLabelText("Leírás"), {
                target: { value: "Kept" },
            });
            fireEvent.click(
                container.querySelectorAll("button:has(svg.lucide-plus)")[
                    index
                ],
            );
            const modal = screen.getByRole("dialog", { name: dialog });
            fireEvent.change(within(modal).getByLabelText(nameField), {
                target: { value: "New" },
            });
            fireEvent.click(
                within(modal).getByRole("button", { name: "Létrehozás" }),
            );
            await waitFor(() =>
                expect(
                    screen.queryByRole("dialog", { name: dialog }),
                ).not.toBeInTheDocument(),
            );
            const input = expect.objectContaining({ id: null });
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: `${label}/create`,
                payload: label === "worksheet" ? { input } : input,
            });
            expect(mocks.dispatch).not.toHaveBeenCalledWith(
                expect.objectContaining({ kind: `${label}/get` }),
            );
            expect(mocks.dispatch).not.toHaveBeenCalledWith(
                expect.objectContaining({ kind: `${label}/update` }),
            );
            expect(
                mocks.dispatch.mock.calls.filter(
                    ([action]) =>
                        action.kind === "task/list" &&
                        action.payload.list === `${label}s`,
                ),
            ).toHaveLength(2);
            expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
            expect(mocks.navigate).not.toHaveBeenCalled();
            await act(async () => {
                await new Promise((resolve) => setTimeout(resolve, 0));
            });
            fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
            await waitFor(() =>
                expect(mocks.dispatch).toHaveBeenCalledWith({
                    kind: "task/update",
                    payload: {
                        input: expect.objectContaining({ [field]: record.id }),
                    },
                }),
            );
        });

        it("closes on cancel without losing task changes", async () => {
            mocks.id = "task-id";
            const { container } = render(<TasksEdit />);
            await waitFor(() =>
                expect(screen.getByLabelText("Leírás")).toHaveValue(
                    "Original task",
                ),
            );
            fireEvent.change(screen.getByLabelText("Leírás"), {
                target: { value: "Kept" },
            });
            fireEvent.click(
                container.querySelectorAll("button:has(svg.lucide-plus)")[
                    index
                ],
            );
            const modal = screen.getByRole("dialog", { name: dialog });
            fireEvent.click(
                within(modal).getByRole("button", { name: "Mégse" }),
            );
            await waitFor(() =>
                expect(
                    screen.queryByRole("dialog", { name: dialog }),
                ).not.toBeInTheDocument(),
            );
            expect(screen.getByLabelText("Leírás")).toHaveValue("Kept");
            expect(mocks.dispatch).not.toHaveBeenCalledWith(
                expect.objectContaining({ kind: `${label}/create` }),
            );
            expect(mocks.navigate).not.toHaveBeenCalled();
        });

        it("preserves standalone fetch and update behavior", async () => {
            mocks.id = "existing-id";
            render(<Component />);
            await waitFor(() =>
                expect(screen.getByLabelText(nameField)).toHaveValue(initial),
            );
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: `${label}/get`,
                payload:
                    label === "worksheet"
                        ? { uuid: "existing-id" }
                        : "existing-id",
            });
            fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
            await waitFor(() =>
                expect(mocks.navigate).toHaveBeenCalledWith(path),
            );
            const input = expect.objectContaining({ id: "existing-id" });
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: `${label}/update`,
                payload: label === "worksheet" ? { input } : input,
            });
        });

        it("preserves standalone creation", async () => {
            render(<Component />);
            fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
            await waitFor(() =>
                expect(mocks.navigate).toHaveBeenCalledWith(path),
            );
            const input = expect.objectContaining({ id: null });
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: `${label}/create`,
                payload: label === "worksheet" ? { input } : input,
            });
        });
    },
);
