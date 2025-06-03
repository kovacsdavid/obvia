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

import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";
import type { PayloadAction } from "@reduxjs/toolkit";
import * as authApi from "@/services/auth";

interface AuthState {
  login: {
    user: { id: string; email: string } | null;
    token: string | null;
    status: "idle" | "loading" | "succeeded" | "failed",
    error: string | null
    isLoggedIn: boolean;
  },
  register: {
    status: "idle" | "loading" | "succeeded" | "failed",
    error: string | null
  },
}

const initialState: AuthState = {
  login: {
    user: null,
    token: null,
    status: "idle",
    error: null,
    isLoggedIn: false,
  },
  register: {
    status: "idle",
    error: null
  }
};

export const registerUser = createAsyncThunk(
  "auth/registerUser",
  async (userData: authApi.RegisterRequest, { rejectWithValue }) => {
    try {
      const data = await authApi.register(userData);
      return data;
    } catch (error: unknown) {
      if (
        typeof error === "object"
          && error !== null
          && "message" in error
      ) {
        return rejectWithValue(error.message);
      }
      return rejectWithValue("Váratlan hiba történt a regisztráció közben");
    }
  }
);

export const loginUser = createAsyncThunk(
  "auth/loginUser",
  async (credentials: authApi.LoginRequest, { rejectWithValue }) => {
    try {
      const data = await authApi.login(credentials);
      return data;
    } catch (error: unknown) {
      if (
        typeof error === "object"
          && error !== null
          && "message" in error
      ) {
        return rejectWithValue(error.message);
      }
      return rejectWithValue("Váratlan hiba történt a bejelentkezés közben");
    }
  }
);

const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    logoutUser(state) {
      state.login.user = null;
      state.login.token = null;
      state.login.status = "idle";
      state.login.error = null;
      state.login.isLoggedIn = false;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(loginUser.pending, (state) => {
        state.login.status = "loading";
        state.login.error = null;
      })
      .addCase(loginUser.fulfilled, (state, action: PayloadAction<authApi.LoginResponse>) => {
        state.login.status = "succeeded";
        state.login.user = action.payload.data.user;
        state.login.token = action.payload.data.token;
        state.login.error = null;
        state.login.isLoggedIn = true;
      })
      .addCase(loginUser.rejected, (state, action) => {
        state.login.status = "failed";
        state.login.error = action.payload as string;
        state.login.isLoggedIn = false;
      });
    builder
      .addCase(registerUser.pending, (state) => {
        state.register.status = "loading";
        state.register.error = null;
      })
      .addCase(registerUser.fulfilled, (state) => {
        state.register.status = "succeeded";
        state.register.error = null;
      })
      .addCase(registerUser.rejected, (state, action) => {
        state.register.status = "failed";
        state.register.error = action.payload as string;
      });
  },
});

export const { logoutUser } = authSlice.actions;
export default authSlice.reducer;
