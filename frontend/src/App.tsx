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
import Create from "@/components/organizational_units/Create.tsx";

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
          path="/szervezeti_egyseg/letrehozas"
          element={
            <PrivateRoute>
              <Create/>
            </PrivateRoute>
          }
        />
        <Route path="*" element={<Navigate to="/vezerlopult"/>}/>
      </Routes>
    </Layout>
  );
}
