/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
import React, { Suspense, lazy } from "react";
import { Navigate, Route, Routes } from "react-router";
import { Layout } from "@/components/layout/Layout";
import type { RootState } from "./store";
import { useAppSelector } from "./store/hooks";

const Login = lazy(() => import("@/components/modules/auth/Login"));
const Register = lazy(() => import("@/components/modules/auth/Register"));
const ForgottenPassword = lazy(
    () => import("@/components/modules/auth/ForgottenPassword"),
);
const EmailVerification = lazy(
    () => import("@/components/modules/auth/EmailVerification"),
);
const Dashboard = lazy(
    () => import("@/components/modules/dashboard/Dashboard"),
);
const Settings = lazy(() => import("@/components/modules/settings/Settings"));

const CustomerEdit = lazy(() => import("@/components/modules/customers/Edit"));
const CustomerList = lazy(() => import("@/components/modules/customers/List"));
const CustomerView = lazy(() => import("@/components/modules/customers/View"));

const InventoryEdit = lazy(() => import("@/components/modules/inventory/Edit"));
const InventoryList = lazy(() => import("@/components/modules/inventory/List"));
const InventoryView = lazy(() => import("@/components/modules/inventory/View"));

const InventoryMovementsEdit = lazy(
    () => import("@/components/modules/inventory_movements/Edit"),
);
const InventoryMovementsList = lazy(
    () => import("@/components/modules/inventory_movements/List"),
);
const InventoryMovementsView = lazy(
    () => import("@/components/modules/inventory_movements/View"),
);

const InventoryReservationsEdit = lazy(
    () => import("@/components/modules/inventory_reservations/Edit"),
);
const InventoryReservationsList = lazy(
    () => import("@/components/modules/inventory_reservations/List"),
);
const InventoryReservationsView = lazy(
    () => import("@/components/modules/inventory_reservations/View"),
);

const ProductsEdit = lazy(() => import("@/components/modules/products/Edit"));
const ProductsList = lazy(() => import("@/components/modules/products/List"));
const ProductsView = lazy(() => import("@/components/modules/products/View"));

const TasksEdit = lazy(() => import("@/components/modules/tasks/Edit"));
const TasksList = lazy(() => import("@/components/modules/tasks/List"));
const TasksView = lazy(() => import("@/components/modules/tasks/View"));

const WarehousesEdit = lazy(
    () => import("@/components/modules/warehouses/Edit"),
);
const WarehousesList = lazy(
    () => import("@/components/modules/warehouses/List"),
);
const WarehousesView = lazy(
    () => import("@/components/modules/warehouses/View"),
);

const WorksheetsEdit = lazy(
    () => import("@/components/modules/worksheets/Edit"),
);
const WorksheetsList = lazy(
    () => import("@/components/modules/worksheets/List"),
);
const WorksheetsView = lazy(
    () => import("@/components/modules/worksheets/View"),
);

const TenantsEdit = lazy(() => import("@/components/modules/databases/Edit"));
const TenantsList = lazy(() => import("@/components/modules/databases/List"));
const TenantsView = lazy(() => import("@/components/modules/databases/View"));

const TaxesEdit = lazy(() => import("@/components/modules/taxes/Edit"));
const TaxesList = lazy(() => import("@/components/modules/taxes/List"));
const TaxesView = lazy(() => import("@/components/modules/taxes/View"));

const ServicesEdit = lazy(() => import("@/components/modules/services/Edit"));
const ServicesList = lazy(() => import("@/components/modules/services/List"));
const ServicesView = lazy(() => import("@/components/modules/services/View"));

type RouteComponent = React.ComponentType<object>;

interface RouteConfig {
    path: string;
    element: RouteComponent;
    isPrivate?: boolean;
}

function crudRoutes(
    basePath: string,
    Edit: RouteComponent,
    List: RouteComponent,
    View: RouteComponent,
): RouteConfig[] {
    return [
        { path: `/${basePath}/letrehozas`, element: Edit, isPrivate: true },
        { path: `/${basePath}/modositas/:id`, element: Edit, isPrivate: true },
        { path: `/${basePath}/lista`, element: List, isPrivate: true },
        { path: `/${basePath}/reszletek/:id`, element: View, isPrivate: true },
    ];
}

const ROUTE_CONFIGS: RouteConfig[] = [
    {
        path: "/",
        element: () => <Navigate to="/bejelentkezes" replace />,
        isPrivate: false,
    },
    { path: "/bejelentkezes", element: Login, isPrivate: false },
    { path: "/regisztracio", element: Register, isPrivate: false },
    {
        path: "/email_megerosites/:id",
        element: EmailVerification,
        isPrivate: false,
    },
    {
        path: "/elfelejtett_jelszo",
        element: ForgottenPassword,
        isPrivate: false,
    },
    {
        path: "/elfelejtett_jelszo/:id",
        element: ForgottenPassword,
        isPrivate: false,
    },

    { path: "/felhasznalo/beallitasok", element: Settings, isPrivate: true },
    { path: "/vezerlopult", element: Dashboard, isPrivate: true },

    ...crudRoutes("adatbazis", TenantsEdit, TenantsList, TenantsView),
    ...crudRoutes("vevo", CustomerEdit, CustomerList, CustomerView),
    ...crudRoutes("raktarkeszlet", InventoryEdit, InventoryList, InventoryView),
    ...crudRoutes("termek", ProductsEdit, ProductsList, ProductsView),
    ...crudRoutes("feladat", TasksEdit, TasksList, TasksView),
    ...crudRoutes("raktar", WarehousesEdit, WarehousesList, WarehousesView),
    ...crudRoutes("munkalap", WorksheetsEdit, WorksheetsList, WorksheetsView),
    ...crudRoutes("ado", TaxesEdit, TaxesList, TaxesView),
    ...crudRoutes("szolgaltatas", ServicesEdit, ServicesList, ServicesView),

    {
        path: "/raktarkeszlet-mozgas/letrehozas",
        element: InventoryMovementsEdit,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-mozgas/letrehozas/:inventoryId",
        element: InventoryMovementsEdit,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-mozgas/modositas/:id",
        element: InventoryMovementsEdit,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-mozgas/lista/:inventoryId",
        element: InventoryMovementsList,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-mozgas/reszletek/:id",
        element: InventoryMovementsView,
        isPrivate: true,
    },

    {
        path: "/raktarkeszlet-foglalas/letrehozas",
        element: InventoryReservationsEdit,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-foglalas/letrehozas/:inventoryId",
        element: InventoryReservationsEdit,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-foglalas/modositas/:id",
        element: InventoryReservationsEdit,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-foglalas/lista/:inventoryId",
        element: InventoryReservationsList,
        isPrivate: true,
    },
    {
        path: "/raktarkeszlet-foglalas/reszletek/:id",
        element: InventoryReservationsView,
        isPrivate: true,
    },

    {
        path: "/munkalap/:referenceId/raktarkeszlet-mozgas/letrehozas",
        element: () => <InventoryMovementsEdit referenceType="worksheets" />,
        isPrivate: true,
    },
];

function PrivateRoute({ children }: { children: React.JSX.Element }) {
    const user = useAppSelector((state: RootState) => state.auth.login.user);
    return user ? children : <Navigate to="/bejelentkezes" replace />;
}

function RouteLoader() {
    return <div>Betöltés...</div>;
}

function createRouteElement(
    Component: RouteComponent,
    isPrivate: boolean,
): React.JSX.Element {
    const element = <Component />;
    return isPrivate ? <PrivateRoute>{element}</PrivateRoute> : element;
}

export default function App() {
    return (
        <Layout>
            <Suspense fallback={<RouteLoader />}>
                <Routes>
                    {ROUTE_CONFIGS.map(
                        ({ path, element: Component, isPrivate = true }) => (
                            <Route
                                key={path}
                                path={path}
                                element={createRouteElement(
                                    Component,
                                    isPrivate,
                                )}
                            />
                        ),
                    )}
                </Routes>
            </Suspense>
        </Layout>
    );
}
