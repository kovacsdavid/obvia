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

import {
  createAsyncThunk,
  createSlice,
  type PayloadAction,
} from "@reduxjs/toolkit";
import * as authApi from "@/components/modules/auth/lib/service.ts";
import type {
  Claims,
  ForgottenPasswordRequest,
  LoginRequest,
  NewPasswordRequest,
  RegisterRequest,
} from "@/components/modules/auth/lib/interface.ts";
import type { RootState } from "@/store";
import type { NewTokenResponse } from "@/components/modules/databases/lib/interface.ts";

interface User {
  id: string;
  email: string;
  first_name: string | null;
  last_name: string | null;
  status: string;
  profile_picture_url: string | null;
}

interface AuthState {
  login: {
    user: User | null;
    claims: Claims | null;
    token: string | null;
    status: "idle" | "loading" | "succeeded" | "failed";
    isLoggedIn: boolean;
    hasActiveDatabase: boolean;
  };
  register: {
    status: "idle" | "loading" | "succeeded" | "failed";
  };
}

const initialState: AuthState = {
  login: {
    claims: null,
    user: null,
    token: null,
    status: "idle",
    isLoggedIn: false,
    hasActiveDatabase: false,
  },
  register: {
    status: "idle",
  },
};

export const registerUserRequest = createAsyncThunk(
  "auth/registerUserRequest",
  async (userData: RegisterRequest) => {
    return await authApi.register(userData);
  },
);

export const loginUserRequest = createAsyncThunk(
  "auth/loginUserRequest",
  async (credentials: LoginRequest) => {
    return await authApi.login(credentials);
  },
);

let refreshInFlight: Promise<void> | null = null;

export const refreshAccessToken = createAsyncThunk(
  "auth/refreshAccessToken",
  async (_: void, { getState, dispatch }) => {
    const rootState = getState() as RootState;

    const token = rootState.auth.login.token;
    const expiresAt = rootState.auth.login.claims?.exp;

    const expiredOrNearExpiry =
      !token || !expiresAt || expiresAt - Math.floor(Date.now() / 1000) < 30;

    if (!expiredOrNearExpiry) return;

    if (!refreshInFlight) {
      refreshInFlight = (async () => {
        try {
          const response = await authApi.refresh();
          if (
            response.statusCode === 200 &&
            typeof response.jsonData?.data !== "undefined"
          ) {
            dispatch(loginUser(response.jsonData?.data));
          } else {
            dispatch(logoutUser());
          }
        } finally {
          refreshInFlight = null;
        }
      })();
    }
    await refreshInFlight;
  },
);

export const get_claims = createAsyncThunk(
  "auth/get_claims",
  async (_, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await authApi.get_claims(token);
  },
);

export const verfiy_email = createAsyncThunk(
  "auth/verfiy_email",
  async (id: string) => {
    return await authApi.verfiy_email(id);
  },
);

export const forgottenPassword = createAsyncThunk(
  "auth/forgottenPassword",
  async (forgottenPasswordRequest: ForgottenPasswordRequest) => {
    return await authApi.forgottenPassword(forgottenPasswordRequest);
  },
);

export const newPassword = createAsyncThunk(
  "auth/newPassword",
  async (newPasswordRequest: NewPasswordRequest) => {
    return await authApi.newPassword(newPasswordRequest);
  },
);

interface LoginUser {
  token: string;
  user: User;
  claims: Claims;
}

const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    loginUser(state, action: PayloadAction<LoginUser>) {
      state.login.claims = action.payload.claims;
      state.login.user = action.payload.user;
      state.login.token = action.payload.token;
      state.login.isLoggedIn = true;
      state.login.hasActiveDatabase =
        action.payload.claims.active_tenant !== null;
    },
    logoutUser(state) {
      state.login.claims = null;
      state.login.user = null;
      state.login.token = null;
      state.login.status = "idle";
      state.login.isLoggedIn = false;
      state.login.hasActiveDatabase = false;
    },
    updateToken(state, action: PayloadAction<NewTokenResponse>) {
      state.login.token = action.payload.token;
      state.login.claims = action.payload.claims;
      state.login.hasActiveDatabase =
        action.payload.claims.active_tenant !== null;
    },
  },
});

export const { logoutUser, updateToken, loginUser } = authSlice.actions;
export default authSlice.reducer;
