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

import { Route, Routes, Navigate } from "react-router-dom";
import React from "react";
import Login from "@/components/auth/Login";
import Register from "@/components/auth/Register";
import Dashboard from "@/components/dashboard/Dashboard";
import { useSelector } from "react-redux";
import type { RootState } from "./store";
import { Layout } from "@/components/layout/Layout";
import {default as CustomerCreate} from "@/components/customers/Create.tsx";
import {default as CustomerList} from "@/components/customers/List.tsx";
import {default as InventoryCreate} from "@/components/inventory/Create.tsx";
import {default as InventoryList} from "@/components/inventory/List.tsx";
import {default as ProductsCreate} from "@/components/products/Create.tsx";
import {default as ProductsList} from "@/components/products/List.tsx";
import {default as ProjectsCreate} from "@/components/projects/Create.tsx";
import {default as ProjectsList} from "@/components/projects/List.tsx";
import {default as TagsCreate} from "@/components/tags/Create.tsx";
import {default as TagsList} from "@/components/tags/List.tsx";
import {default as TasksCreate} from "@/components/tasks/Create.tsx";
import {default as TasksList} from "@/components/tasks/List.tsx";
import {default as UsersCreate} from "@/components/users/Create.tsx";
import {default as UsersList} from "@/components/users/List.tsx";
import {default as WarehousesCreate} from "@/components/warehouses/Create.tsx";
import {default as WarehousesList} from "@/components/warehouses/List.tsx";
import {default as WorksheetsCreate} from "@/components/worksheets/Create.tsx";
import {default as WorksheetsList} from "@/components/worksheets/List.tsx";
import {default as TenantsCreate} from "@/components/tenants/Create.tsx";
import {default as TenantsList}  from "@/components/tenants/List.tsx";

function PrivateRoute({ children }: { children: React.JSX.Element }) {
  const user = useSelector((state: RootState) => state.auth.login.user);
  return user ? children : <Navigate to="/bejelentkezes" replace />;
}

export default function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/bejelentkezes" element={<Login/>}/>
        <Route path="/regisztracio" element={<Register/>}/>
        <Route
          path="/vezerlopult"
          element={
            <PrivateRoute>
              <Dashboard/>
            </PrivateRoute>
          }
        />
        <Route
          path="/szervezeti_egyseg/uj"
          element={
            <PrivateRoute>
              <TenantsCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/szervezeti_egyseg/lista"
          element={
            <PrivateRoute>
              <TenantsList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/vevo/uj"
          element={
            <PrivateRoute>
              <CustomerCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/vevo/lista"
          element={
            <PrivateRoute>
              <CustomerList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/leltar/uj"
          element={
            <PrivateRoute>
              <InventoryCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/leltar/lista"
          element={
            <PrivateRoute>
              <InventoryList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/termek/uj"
          element={
            <PrivateRoute>
              <ProductsCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/termek/lista"
          element={
            <PrivateRoute>
              <ProductsList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/projekt/uj"
          element={
            <PrivateRoute>
              <ProjectsCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/projekt/lista"
          element={
            <PrivateRoute>
              <ProjectsList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/cimke/uj"
          element={
            <PrivateRoute>
              <TagsCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/cimke/lista"
          element={
            <PrivateRoute>
              <TagsList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/feladat/uj"
          element={
            <PrivateRoute>
              <TasksCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/feladat/lista"
          element={
            <PrivateRoute>
              <TasksList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/felhasznalo/uj"
          element={
            <PrivateRoute>
              <UsersCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/felhasznalo/lista"
          element={
            <PrivateRoute>
              <UsersList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/raktar/uj"
          element={
            <PrivateRoute>
              <WarehousesCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/raktar/lista"
          element={
            <PrivateRoute>
              <WarehousesList/>
            </PrivateRoute>
          }
        />
        <Route
          path="/munkalap/uj"
          element={
            <PrivateRoute>
              <WorksheetsCreate/>
            </PrivateRoute>
          }
        />
        <Route
          path="/munkalap/lista"
          element={
            <PrivateRoute>
              <WorksheetsList/>
            </PrivateRoute>
          }
        />
        <Route path="*" element={<Navigate to="/vezerlopult"/>}/>
      </Routes>
    </Layout>
  );
}
