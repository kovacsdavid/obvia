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
import Login from "@/components/auth/Login";
import Register from "@/components/auth/Register";
import Dashboard from "@/components/dashboard/Dashboard";
import {useSelector} from "react-redux";
import type {RootState} from "./store";
import {Layout} from "@/components/layout/Layout";
import CustomerEdit from "@/components/customers/Edit.tsx";
import CustomerList from "@/components/customers/List.tsx";
import CustomerView from "@/components/customers/View.tsx";
import InventoryEdit from "@/components/inventory/Edit.tsx";
import InventoryList from "@/components/inventory/List.tsx";
import InventoryView from "@/components/inventory/View.tsx";
import ProductsEdit from "@/components/products/Edit.tsx";
import ProductsList from "@/components/products/List.tsx";
import ProductsView from "@/components/products/View.tsx";
import ProjectsEdit from "@/components/projects/Edit.tsx";
import ProjectsList from "@/components/projects/List.tsx";
import ProjectsView from "@/components/projects/View.tsx";
import TagsEdit from "@/components/tags/Edit.tsx";
import TagsList from "@/components/tags/List.tsx";
import TagsView from "@/components/tags/View.tsx";
import TasksEdit from "@/components/tasks/Edit.tsx";
import TasksList from "@/components/tasks/List.tsx";
import TasksView from "@/components/tasks/View.tsx";
//import UsersEdit from "@/components/users/Edit.tsx";
//import UsersList from "@/components/users/List.tsx";
import WarehousesEdit from "@/components/warehouses/Edit.tsx";
import WarehousesList from "@/components/warehouses/List.tsx";
import WarehousesView from "@/components/warehouses/View.tsx";
import WorksheetsEdit from "@/components/worksheets/Edit.tsx";
import WorksheetsList from "@/components/worksheets/List.tsx";
import WorksheetsView from "@/components/worksheets/View.tsx";
import TenantsEdit from "@/components/tenants/Edit.tsx";
import TenantsList from "@/components/tenants/List.tsx";
import TenantsView from "@/components/tenants/View.tsx";

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
