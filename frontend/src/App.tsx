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

import {Navigate, Route, Routes} from "react-router-dom";
import React from "react";
import Login from "@/components/modules/auth/Login";
import Register from "@/components/modules/auth/Register";
import Dashboard from "@/components/modules/dashboard/Dashboard";
import {useSelector} from "react-redux";
import type {RootState} from "./store";
import {Layout} from "@/components/layout/Layout";
import CustomerEdit from "@/components/modules/customers/Edit.tsx";
import CustomerList from "@/components/modules/customers/List.tsx";
import CustomerView from "@/components/modules/customers/View.tsx";
import InventoryEdit from "@/components/modules/inventory/Edit.tsx";
import InventoryList from "@/components/modules/inventory/List.tsx";
import InventoryView from "@/components/modules/inventory/View.tsx";
import InventoryMovementsEdit from "@/components/modules/inventory_movements/Edit.tsx";
import InventoryMovementsList from "@/components/modules/inventory_movements/List.tsx";
import InventoryMovementsView from "@/components/modules/inventory_movements/View.tsx";
import InventoryReservationsEdit from "@/components/modules/inventory_reservations/Edit.tsx";
import InventoryReservationsList from "@/components/modules/inventory_reservations/List.tsx";
import InventoryReservationsView from "@/components/modules/inventory_reservations/View.tsx";
import ProductsEdit from "@/components/modules/products/Edit.tsx";
import ProductsList from "@/components/modules/products/List.tsx";
import ProductsView from "@/components/modules/products/View.tsx";
import ProjectsEdit from "@/components/modules/projects/Edit.tsx";
import ProjectsList from "@/components/modules/projects/List.tsx";
import ProjectsView from "@/components/modules/projects/View.tsx";
import TagsEdit from "@/components/modules/tags/Edit.tsx";
import TagsList from "@/components/modules/tags/List.tsx";
import TagsView from "@/components/modules/tags/View.tsx";
import TasksEdit from "@/components/modules/tasks/Edit.tsx";
import TasksList from "@/components/modules/tasks/List.tsx";
import TasksView from "@/components/modules/tasks/View.tsx";
//import UsersEdit from "@/components/users/Edit.tsx";
//import UsersList from "@/components/users/List.tsx";
import WarehousesEdit from "@/components/modules/warehouses/Edit.tsx";
import WarehousesList from "@/components/modules/warehouses/List.tsx";
import WarehousesView from "@/components/modules/warehouses/View.tsx";
import WorksheetsEdit from "@/components/modules/worksheets/Edit.tsx";
import WorksheetsList from "@/components/modules/worksheets/List.tsx";
import WorksheetsView from "@/components/modules/worksheets/View.tsx";
import TenantsEdit from "@/components/modules/databases/Edit.tsx";
import TenantsList from "@/components/modules/databases/List.tsx";
import TenantsView from "@/components/modules/databases/View.tsx";
import TaxesEdit from "@/components/modules/taxes/Edit.tsx";
import TaxesList from "@/components/modules/taxes/List.tsx";
import TaxesView from "@/components/modules/taxes/View.tsx";
import ServicesEdit from "@/components/modules/services/Edit.tsx";
import ServicesList from "@/components/modules/services/List.tsx";
import ServicesView from "@/components/modules/services/View.tsx";
import EmailVerification from "@/components/modules/auth/EmailVerification.tsx";

interface RouteConfig {
  path: string;
  element: React.ComponentType;
  isPrivate?: boolean;
}

const ROUTE_CONFIGS: RouteConfig[] = [
  // Public routes
  {path: "/", element: (): React.JSX.Element => (<Navigate to="/bejelentkezes" replace/>), isPrivate: false},
  {path: "/bejelentkezes", element: Login, isPrivate: false},
  {path: "/regisztracio", element: Register, isPrivate: false},
  {path: "/email_megerosites/:id", element: EmailVerification, isPrivate: false},

  // Dashboard
  {path: "/vezerlopult", element: Dashboard, isPrivate: true},

  // Tenants
  {path: "/adatbazis/letrehozas", element: TenantsEdit, isPrivate: true},
  {path: "/adatbazis/lista", element: TenantsList, isPrivate: true},
  {path: "/adatbazis/reszletek/:id", element: TenantsView, isPrivate: true},

  // Customers
  {path: "/vevo/letrehozas", element: CustomerEdit, isPrivate: true},
  {path: "/vevo/modositas/:id", element: CustomerEdit, isPrivate: true},
  {path: "/vevo/lista", element: CustomerList, isPrivate: true},
  {path: "/vevo/reszletek/:id", element: CustomerView, isPrivate: true},

  // Inventory
  {path: "/raktarkeszlet/letrehozas", element: InventoryEdit, isPrivate: true},
  {path: "/raktarkeszlet/modositas/:id", element: InventoryEdit, isPrivate: true},
  {path: "/raktarkeszlet/lista", element: InventoryList, isPrivate: true},
  {path: "/raktarkeszlet/reszletek/:id", element: InventoryView, isPrivate: true},

  // InventoryMovements
  {path: "/raktarkeszlet-mozgas/letrehozas", element: InventoryMovementsEdit, isPrivate: true},
  {path: "/raktarkeszlet-mozgas/letrehozas/:inventoryId", element: InventoryMovementsEdit, isPrivate: true},
  {path: "/raktarkeszlet-mozgas/modositas/:id", element: InventoryMovementsEdit, isPrivate: true},
  {path: "/raktarkeszlet-mozgas/lista", element: InventoryMovementsList, isPrivate: true},
  {path: "/raktarkeszlet-mozgas/reszletek/:id", element: InventoryMovementsView, isPrivate: true},

  // InventoryReservations
  {path: "/raktarkeszlet-foglalas/letrehozas", element: InventoryReservationsEdit, isPrivate: true},
  {path: "/raktarkeszlet-foglalas/letrehozas/:inventoryId", element: InventoryReservationsEdit, isPrivate: true},
  {path: "/raktarkeszlet-foglalas/modositas/:id", element: InventoryReservationsEdit, isPrivate: true},
  {path: "/raktarkeszlet-foglalas/lista", element: InventoryReservationsList, isPrivate: true},
  {path: "/raktarkeszlet-foglalas/reszletek/:id", element: InventoryReservationsView, isPrivate: true},

  // Products
  {path: "/termek/letrehozas", element: ProductsEdit, isPrivate: true},
  {path: "/termek/modositas/:id", element: ProductsEdit, isPrivate: true},
  {path: "/termek/lista", element: ProductsList, isPrivate: true},
  {path: "/termek/reszletek/:id", element: ProductsView, isPrivate: true},

  // Projects
  {path: "/projekt/letrehozas", element: ProjectsEdit, isPrivate: true},
  {path: "/projekt/modositas/:id", element: ProjectsEdit, isPrivate: true},
  {path: "/projekt/lista", element: ProjectsList, isPrivate: true},
  {path: "/projekt/reszletek/:id", element: ProjectsView, isPrivate: true},

  // Tags
  {path: "/cimke/letrehozas", element: TagsEdit, isPrivate: true},
  {path: "/cimke/modositas/:id", element: TagsEdit, isPrivate: true},
  {path: "/cimke/lista", element: TagsList, isPrivate: true},
  {path: "/cimke/reszletek/:id", element: TagsView, isPrivate: true},

  // Tasks
  {path: "/feladat/letrehozas", element: TasksEdit, isPrivate: true},
  {path: "/feladat/modositas/:id", element: TasksEdit, isPrivate: true},
  {path: "/feladat/lista", element: TasksList, isPrivate: true},
  {path: "/feladat/reszletek/:id", element: TasksView, isPrivate: true},

  // Users
  // {path: "/felhasznalo/modositas", element: UsersEdit, isPrivate: true},
  // {path: "/felhasznalo/modositas/:id", element: UsersEdit, isPrivate: true},
  // {path: "/felhasznalo/lista", element: UsersList, isPrivate: true},

  // Warehouses
  {path: "/raktar/letrehozas", element: WarehousesEdit, isPrivate: true},
  {path: "/raktar/modositas/:id", element: WarehousesEdit, isPrivate: true},
  {path: "/raktar/lista", element: WarehousesList, isPrivate: true},
  {path: "/raktar/reszletek/:id", element: WarehousesView, isPrivate: true},

  // Worksheets
  {path: "/munkalap/letrehozas", element: WorksheetsEdit, isPrivate: true},
  {path: "/munkalap/modositas/:id", element: WorksheetsEdit, isPrivate: true},
  {path: "/munkalap/lista", element: WorksheetsList, isPrivate: true},
  {path: "/munkalap/reszletek/:id", element: WorksheetsView, isPrivate: true},

  // Taxes
  {path: "/ado/letrehozas", element: TaxesEdit, isPrivate: true},
  {path: "/ado/modositas/:id", element: TaxesEdit, isPrivate: true},
  {path: "/ado/lista", element: TaxesList, isPrivate: true},
  {path: "/ado/reszletek/:id", element: TaxesView, isPrivate: true},

  // Services
  {path: "/szolgaltatas/letrehozas", element: ServicesEdit, isPrivate: true},
  {path: "/szolgaltatas/modositas/:id", element: ServicesEdit, isPrivate: true},
  {path: "/szolgaltatas/lista", element: ServicesList, isPrivate: true},
  {path: "/szolgaltatas/reszletek/:id", element: ServicesView, isPrivate: true},
];

function PrivateRoute({children}: { children: React.JSX.Element }) {
  const user = useSelector((state: RootState) => state.auth.login.user);
  return user ? children : <Navigate to="/bejelentkezes" replace/>;
}

function createRouteElement(Component: React.ComponentType, isPrivate: boolean): React.JSX.Element {
  const element = <Component/>;
  return isPrivate ? <PrivateRoute>{element}</PrivateRoute> : element;
}

export default function App() {
  return (
    <Layout>
      <Routes>
        {ROUTE_CONFIGS.map(({path, element: Component, isPrivate = true}) => (
          <Route
            key={path}
            path={path}
            element={createRouteElement(Component, isPrivate)}
          />
        ))}
      </Routes>
    </Layout>
  );
}
