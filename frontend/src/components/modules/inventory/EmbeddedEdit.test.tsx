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
import InventoryEdit from "./Edit";
import MovementsEdit from "../inventory_movements/Edit";
import ReservationsEdit from "../inventory_reservations/Edit";

const mocks = vi.hoisted(() => {
    const action = (kind: string) =>
        Object.assign((payload: unknown) => ({ kind, payload }), {
            fulfilled: {
                match: (response: { kind: string }) => response.kind === kind,
            },
        });
    const actions = (prefix: string) => ({
        get: action(`${prefix}/get`),
        create: action(`${prefix}/create`),
        update: action(`${prefix}/update`),
        select_list: action(`${prefix}/list`),
    });
    return {
        id: undefined as string | undefined,
        dispatch: vi.fn(),
        navigate: vi.fn(),
        logout: vi.fn(),
        inventory: actions("inventory"),
        movement: actions("movement"),
        reservation: actions("reservation"),
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
vi.mock("@/components/modules/inventory/lib/slice.ts", () => mocks.inventory);
vi.mock(
    "@/components/modules/inventory_movements/lib/slice.ts",
    () => mocks.movement,
);
vi.mock(
    "@/components/modules/inventory_reservations/lib/slice.ts",
    () => mocks.reservation,
);
vi.mock("@/components/modules/products/Edit.tsx", () => ({
    default: () => null,
}));
vi.mock("@/components/modules/warehouses/Edit.tsx", () => ({
    default: () => null,
}));

const parentId = "01234567-89ab-cdef-0123-456789abcdef";
const inventory = {
    id: "new-inventory",
    product_id: "product",
    warehouse_id: "warehouse",
    minimum_stock: null,
    maximum_stock: null,
    currency_code: "HUF",
    status: "active",
};
const parent = {
    inventory_id: "old-inventory",
    movement_type: "in",
    quantity: "3",
    unit_price: "1",
    tax_id: "tax",
    reference_type: "",
    reference_id: "",
    reserved_until: null,
    status: "active",
};

beforeEach(() => {
    mocks.id = undefined;
    mocks.dispatch.mockReset();
    mocks.navigate.mockReset();
    mocks.dispatch.mockImplementation(
        (action: { kind: string; payload: unknown }) => {
            const data =
                action.kind === "inventory/get" ||
                action.kind === "inventory/create"
                    ? inventory
                    : action.kind === "movement/get" ||
                        action.kind === "reservation/get"
                      ? parent
                      : (action.kind === "movement/list" ||
                              action.kind === "reservation/list") &&
                          action.payload === "inventory"
                        ? [
                              {
                                  value: "new-inventory",
                                  title: "New inventory",
                              },
                              {
                                  value: "old-inventory",
                                  title: "Old inventory",
                              },
                          ]
                        : [];
            return Promise.resolve({
                kind: action.kind,
                payload: {
                    statusCode: action.kind.endsWith("/create") ? 201 : 200,
                    jsonData: { data },
                },
            });
        },
    );
});
afterEach(cleanup);

describe("inventory editor mode", () => {
    it("fetches and updates on the standalone edit route", async () => {
        mocks.id = parentId;
        render(<InventoryEdit />);
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "inventory/get",
                payload: parentId,
            }),
        );
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/raktarkeszlet/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "inventory/update",
            payload: expect.objectContaining({ id: parentId }),
        });
    });

    it("continues to create on the standalone creation route", async () => {
        render(<InventoryEdit />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() =>
            expect(mocks.navigate).toHaveBeenCalledWith("/raktarkeszlet/lista"),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "inventory/create",
            payload: expect.objectContaining({ id: null }),
        });
    });
});

describe.each([
    { label: "movement", Component: MovementsEdit },
    { label: "reservation", Component: ReservationsEdit },
])("$label inventory dialog", ({ label, Component }) => {
    it("creates inventory without inheriting the parent ID and selects it", async () => {
        mocks.id = parentId;
        const { container } = render(<Component />);
        await waitFor(() =>
            expect(screen.getByLabelText("Mennyiség")).toHaveValue("3,00"),
        );
        fireEvent.change(screen.getByLabelText("Mennyiség"), {
            target: { value: "42" },
        });
        fireEvent.click(
            container.querySelector("button:has(svg.lucide-plus)")!,
        );
        const modal = screen.getByRole("dialog", {
            name: "Raktárkészlet létrehozása",
        });
        fireEvent.click(
            within(modal).getByRole("button", { name: "Létrehozás" }),
        );
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", {
                    name: "Raktárkészlet létrehozása",
                }),
            ).not.toBeInTheDocument(),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: "inventory/create",
            payload: expect.objectContaining({ id: null }),
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "inventory/get" }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "inventory/update" }),
        );
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) =>
                    action.kind === `${label}/list` &&
                    action.payload === "inventory",
            ),
        ).toHaveLength(2);
        expect(screen.getByLabelText("Mennyiség")).toHaveValue("42,00");
        expect(mocks.navigate).not.toHaveBeenCalled();
        await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, 0));
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: `${label}/create`,
                payload: expect.objectContaining({
                    inventoryId: "new-inventory",
                }),
            }),
        );
    });

    it("cancels without losing parent changes", async () => {
        mocks.id = parentId;
        const { container } = render(<Component />);
        await waitFor(() =>
            expect(screen.getByLabelText("Mennyiség")).toHaveValue("3,00"),
        );
        fireEvent.change(screen.getByLabelText("Mennyiség"), {
            target: { value: "42" },
        });
        fireEvent.click(
            container.querySelector("button:has(svg.lucide-plus)")!,
        );
        const modal = screen.getByRole("dialog", {
            name: "Raktárkészlet létrehozása",
        });
        fireEvent.click(within(modal).getByRole("button", { name: "Mégse" }));
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", {
                    name: "Raktárkészlet létrehozása",
                }),
            ).not.toBeInTheDocument(),
        );
        expect(screen.getByLabelText("Mennyiség")).toHaveValue("42,00");
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: "inventory/create" }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});
