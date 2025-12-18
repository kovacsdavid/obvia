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

import type { ReactNode } from "react";
import { createContext, useContext } from "react";
import { useSelector } from "react-redux";
import type { RootState } from "@/store";
import { useAppDispatch } from "@/store/hooks";
import {
  loginUserRequest,
  logoutAndRevokeRefreshToken,
} from "@/components/modules/auth/lib/slice.ts";
import type { PayloadAction } from "@reduxjs/toolkit";

type AuthContextType = {
  isLoggedIn: boolean;
  hasActiveDatabase: boolean;
  login: (
    email: string,
    password: string,
  ) => Promise<PayloadAction<any, any, any>>;
  logout: () => void;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const dispach = useAppDispatch();
  const isLoggedIn = useSelector(
    (state: RootState) => state.auth.login.isLoggedIn,
  );
  const hasActiveDatabase = useSelector(
    (state: RootState) => state.auth.login.hasActiveDatabase,
  );

  const login = (email: string, password: string) => {
    return dispach(loginUserRequest({ email, password }));
  };
  const logout = () => {
    dispach(logoutAndRevokeRefreshToken());
  };

  return (
    <AuthContext.Provider
      value={{ isLoggedIn, hasActiveDatabase, login, logout }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within AuthProvider");
  return context;
}
