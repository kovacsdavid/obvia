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
import TenantsEdit from "@/components/modules/tenants/Edit.tsx";
import TenantsList from "@/components/modules/tenants/List.tsx";
import TenantsView from "@/components/modules/tenants/View.tsx";
import TaxesEdit from "@/components/modules/taxes/Edit.tsx";
import TaxesList from "@/components/modules/taxes/List.tsx";
import TaxesView from "@/components/modules/taxes/View.tsx";
import ServicesEdit from "@/components/modules/services/Edit.tsx";
import ServicesList from "@/components/modules/services/List.tsx";
import ServicesView from "@/components/modules/services/View.tsx";

interface RouteConfig {
  path: string;
  element: React.ComponentType;
  isPrivate?: boolean;
}

const ROUTE_CONFIGS: RouteConfig[] = [
  // Public routes
  {path: "/bejelentkezes", element: Login, isPrivate: false},
  {path: "/regisztracio", element: Register, isPrivate: false},

  // Dashboard
  {path: "/vezerlopult", element: Dashboard, isPrivate: true},

  // Tenants
  {path: "/szervezeti_egyseg/szerkesztes", element: TenantsEdit, isPrivate: true},
  {path: "/szervezeti_egyseg/lista", element: TenantsList, isPrivate: true},
  {path: "/szervezeti_egyseg/reszletek/:id", element: TenantsView, isPrivate: true},

  // Customers
  {path: "/vevo/szerkesztes", element: CustomerEdit, isPrivate: true},
  {path: "/vevo/szerkesztes/:id", element: CustomerEdit, isPrivate: true},
  {path: "/vevo/lista", element: CustomerList, isPrivate: true},
  {path: "/vevo/reszletek/:id", element: CustomerView, isPrivate: true},

  // Inventory
  {path: "/leltar/szerkesztes", element: InventoryEdit, isPrivate: true},
  {path: "/leltar/szerkesztes/:id", element: InventoryEdit, isPrivate: true},
  {path: "/leltar/lista", element: InventoryList, isPrivate: true},
  {path: "/leltar/reszletek/:id", element: InventoryView, isPrivate: true},

  // Products
  {path: "/termek/szerkesztes", element: ProductsEdit, isPrivate: true},
  {path: "/termek/szerkesztes/:id", element: ProductsEdit, isPrivate: true},
  {path: "/termek/lista", element: ProductsList, isPrivate: true},
  {path: "/termek/reszletek/:id", element: ProductsView, isPrivate: true},

  // Projects
  {path: "/projekt/szerkesztes", element: ProjectsEdit, isPrivate: true},
  {path: "/projekt/szerkesztes/:id", element: ProjectsEdit, isPrivate: true},
  {path: "/projekt/lista", element: ProjectsList, isPrivate: true},
  {path: "/projekt/reszletek/:id", element: ProjectsView, isPrivate: true},

  // Tags
  {path: "/cimke/szerkesztes", element: TagsEdit, isPrivate: true},
  {path: "/cimke/szerkesztes/:id", element: TagsEdit, isPrivate: true},
  {path: "/cimke/lista", element: TagsList, isPrivate: true},
  {path: "/cimke/reszletek/:id", element: TagsView, isPrivate: true},

  // Tasks
  {path: "/feladat/szerkesztes", element: TasksEdit, isPrivate: true},
  {path: "/feladat/szerkesztes/:id", element: TasksEdit, isPrivate: true},
  {path: "/feladat/lista", element: TasksList, isPrivate: true},
  {path: "/feladat/reszletek/:id", element: TasksView, isPrivate: true},

  // Users
  // {path: "/felhasznalo/szerkesztes", element: UsersEdit, isPrivate: true},
  // {path: "/felhasznalo/szerkesztes/:id", element: UsersEdit, isPrivate: true},
  // {path: "/felhasznalo/lista", element: UsersList, isPrivate: true},

  // Warehouses
  {path: "/raktar/szerkesztes", element: WarehousesEdit, isPrivate: true},
  {path: "/raktar/szerkesztes/:id", element: WarehousesEdit, isPrivate: true},
  {path: "/raktar/lista", element: WarehousesList, isPrivate: true},
  {path: "/raktar/reszletek/:id", element: WarehousesView, isPrivate: true},

  // Worksheets
  {path: "/munkalap/szerkesztes", element: WorksheetsEdit, isPrivate: true},
  {path: "/munkalap/szerkesztes/:id", element: WorksheetsEdit, isPrivate: true},
  {path: "/munkalap/lista", element: WorksheetsList, isPrivate: true},
  {path: "/munkalap/reszletek/:id", element: WorksheetsView, isPrivate: true},

  // Taxes
  {path: "/ado/szerkesztes", element: TaxesEdit, isPrivate: true},
  {path: "/ado/szerkesztes/:id", element: TaxesEdit, isPrivate: true},
  {path: "/ado/lista", element: TaxesList, isPrivate: true},
  {path: "/ado/reszletek/:id", element: TaxesView, isPrivate: true},

  // Services
  {path: "/szolgaltatas/szerkesztes", element: ServicesEdit, isPrivate: true},
  {path: "/szolgaltatas/szerkesztes/:id", element: ServicesEdit, isPrivate: true},
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
