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
import ProductsEdit from "../products/Edit";
import WarehousesEdit from "../warehouses/Edit";

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
        product: {
            create: action("product/create"),
            get: action("product/get"),
            update: action("product/update"),
            select_list: action("product/list"),
        },
        warehouse: {
            create: action("warehouse/create"),
            get: action("warehouse/get"),
            update: action("warehouse/update"),
        },
        inventory: {
            create: action("inventory/create"),
            get: action("inventory/get"),
            update: action("inventory/update"),
            select_list: action("inventory/list"),
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
vi.mock("@/components/modules/products/lib/slice.ts", () => mocks.product);
vi.mock("@/components/modules/warehouses/lib/slice.ts", () => mocks.warehouse);
vi.mock("@/components/modules/inventory/lib/slice.ts", () => mocks.inventory);

const product = {
    id: "new-product",
    name: "Product",
    description: "",
    unit_of_measure_id: "unit",
    status: "active",
};
const warehouse = {
    id: "new-warehouse",
    name: "Warehouse",
    contact_name: null,
    contact_phone: null,
    status: "active",
};

beforeEach(() => {
    mocks.id = undefined;
    mocks.dispatch.mockReset();
    mocks.navigate.mockReset();
    mocks.dispatch.mockImplementation((action: { kind: string }) => {
        const data =
            action.kind === "inventory/get"
                ? {
                      product_id: "old-product",
                      warehouse_id: "old-warehouse",
                      minimum_stock: null,
                      maximum_stock: null,
                      currency_code: "HUF",
                      status: "active",
                  }
                : action.kind.startsWith("product/") &&
                    !action.kind.endsWith("/list")
                  ? product
                  : action.kind.startsWith("warehouse/")
                    ? warehouse
                    : [];
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
        label: "product",
        Component: ProductsEdit,
        data: product,
        path: "/termek/lista",
    },
    {
        label: "warehouse",
        Component: WarehousesEdit,
        data: warehouse,
        path: "/raktar/lista",
    },
])("$label editor mode", ({ label, Component, data, path }) => {
    it("creates with no ID even when the enclosing route has one", async () => {
        mocks.id = "inventory-id";
        const onSuccess = vi.fn();
        render(<Component createOnly onSuccess={onSuccess} />);

        fireEvent.change(screen.getByLabelText("Név"), {
            target: { value: "New name" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(onSuccess).toHaveBeenCalledWith(data));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: `${label}/create`,
            payload: expect.objectContaining({ id: null, name: "New name" }),
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: `${label}/get` }),
        );
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: `${label}/update` }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
    });

    it("preserves standalone fetch and update behavior", async () => {
        mocks.id = "existing-id";
        render(<Component />);
        await waitFor(() =>
            expect(screen.getByLabelText("Név")).toHaveValue(data.name),
        );
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: `${label}/get`,
            payload: "existing-id",
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() => expect(mocks.navigate).toHaveBeenCalledWith(path));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: `${label}/update`,
            payload: expect.objectContaining({ id: "existing-id" }),
        });
    });

    it("continues to create on the standalone creation route", async () => {
        render(<Component />);
        fireEvent.click(screen.getByRole("button", { name: "Létrehozás" }));
        await waitFor(() => expect(mocks.navigate).toHaveBeenCalledWith(path));
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: `${label}/create`,
            payload: expect.objectContaining({ id: null }),
        });
    });

    it("uses the cancel callback without navigating", () => {
        mocks.id = "inventory-id";
        const onCancel = vi.fn();
        render(<Component createOnly onCancel={onCancel} />);
        fireEvent.click(screen.getByRole("button", { name: "Mégse" }));
        expect(onCancel).toHaveBeenCalledOnce();
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});

describe.each([
    {
        label: "product",
        dialog: "Termék létrehozása",
        field: "productId",
        id: "new-product",
        index: 0,
    },
    {
        label: "warehouse",
        dialog: "Raktár létrehozása",
        field: "warehouseId",
        id: "new-warehouse",
        index: 1,
    },
])("inventory $label dialog", ({ label, dialog, field, id, index }) => {
    it("creates and selects the new record without leaving the inventory", async () => {
        mocks.id = "inventory-id";
        const { container } = render(<InventoryEdit />);
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "inventory/get",
                payload: "inventory-id",
            }),
        );
        const button = container.querySelectorAll(
            "button:has(svg.lucide-plus)",
        )[index];
        fireEvent.click(button);
        const modal = screen.getByRole("dialog", { name: dialog });
        fireEvent.change(within(modal).getByLabelText("Név"), {
            target: { value: "Created" },
        });
        fireEvent.click(
            within(modal).getByRole("button", { name: "Létrehozás" }),
        );
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", { name: dialog }),
            ).not.toBeInTheDocument(),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
        expect(mocks.dispatch).toHaveBeenCalledWith({
            kind: `${label}/create`,
            payload: expect.objectContaining({ id: null }),
        });
        expect(
            mocks.dispatch.mock.calls.filter(
                ([action]) =>
                    action.kind === "inventory/list" &&
                    action.payload === `${label}s`,
            ),
        ).toHaveLength(2);
        await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, 0));
        });
        fireEvent.click(screen.getByRole("button", { name: "Módosítás" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "inventory/update",
                payload: expect.objectContaining({ [field]: id }),
            }),
        );
    });

    it("closes on cancel without losing inventory changes", async () => {
        mocks.id = "inventory-id";
        const { container } = render(<InventoryEdit />);
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith({
                kind: "inventory/get",
                payload: "inventory-id",
            }),
        );
        fireEvent.change(screen.getByLabelText("Minimum készlet"), {
            target: { value: "42" },
        });
        fireEvent.click(
            container.querySelectorAll("button:has(svg.lucide-plus)")[index],
        );
        const modal = screen.getByRole("dialog", { name: dialog });
        fireEvent.click(within(modal).getByRole("button", { name: "Mégse" }));
        await waitFor(() =>
            expect(
                screen.queryByRole("dialog", { name: dialog }),
            ).not.toBeInTheDocument(),
        );
        expect(screen.getByLabelText("Minimum készlet")).toHaveValue("42");
        expect(mocks.dispatch).not.toHaveBeenCalledWith(
            expect.objectContaining({ kind: `${label}/create` }),
        );
        expect(mocks.navigate).not.toHaveBeenCalled();
    });
});
