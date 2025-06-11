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
import {isLoginResponse, isRegisterResponse} from "@/services/auth";

interface AuthState {
  login: {
    user: {
      id: string;
      email: string;
      first_name: string | null;
      last_name: string | null;
      status: string;
      profile_picture_url: string | null;
    } | null;
    token: string | null;
    status: "idle" | "loading" | "succeeded" | "failed",
    error: {
      global: string | null,
      fields: Record<string, string | null>
    }
    isLoggedIn: boolean;
  },
  register: {
    status: "idle" | "loading" | "succeeded" | "failed",
    error: {
      global: string | null,
      fields: Record<string, string | null>
    }
  },
}

const initialState: AuthState = {
  login: {
    user: null,
    token: null,
    status: "idle",
    error: {
      global: null,
      fields: {},
    },
    isLoggedIn: false,
  },
  register: {
    status: "idle",
    error: {
      global: null,
      fields: {},
    }
  }
};

export const registerUser = createAsyncThunk(
  "auth/registerUser",
  async (userData: authApi.RegisterRequest, { rejectWithValue }) => {
    try {
      const response = await authApi.register(userData);
      if (response.success) {
        return response;
      } else {
        return rejectWithValue(response);
      }
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
);

export const loginUser = createAsyncThunk(
  "auth/loginUser",
  async (credentials: authApi.LoginRequest, { rejectWithValue }) => {
    try {
      const response = await authApi.login(credentials);
      if (response.success) {
        return response;
      } else {
        return rejectWithValue(response);
      }
    } catch (error: unknown) {
      return rejectWithValue(error);
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
      state.login.error = { global: null, fields: {} };
      state.login.isLoggedIn = false;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(loginUser.pending, (state) => {
        state.login.status = "loading";
        state.login.error = { global: null, fields: {}};
      })
      .addCase(loginUser.fulfilled, (state, action: PayloadAction<authApi.LoginResponse>) => {
        state.login.status = "succeeded";
        if (typeof action.payload.data !== "undefined") {
          state.login.user = action.payload.data.user;
          state.login.token = action.payload.data.token;
        }
        state.login.error = { global: null, fields: {}};
        state.login.isLoggedIn = true;
      })
      .addCase(loginUser.rejected, (state, action) => {
        state.login.status = "failed";
        if (isLoginResponse(action.payload) && typeof action.payload?.error !== "undefined") {
          state.login.error = action.payload.error;
        } else {
          state.login.error = { global: "Váratlan hiba történt a kommunikáció során", fields: {}};
        }
        state.login.isLoggedIn = false;
      });
    builder
      .addCase(registerUser.pending, (state) => {
        state.register.status = "loading";
        state.register.error = { global: null, fields: {}};
      })
      .addCase(registerUser.fulfilled, (state) => {
        state.register.status = "succeeded";
        state.register.error = { global: null, fields: {}};
      })
      .addCase(registerUser.rejected, (state, action) => {
        state.register.status = "failed";
        if (isRegisterResponse(action.payload) && typeof action.payload?.error !== "undefined") {
          state.register.error = action.payload.error;
        } else {
          state.register.error = { global: "Váratlan hiba történt a kommunikáció során", fields: {}};
        }
      });
  },
});

export const { logoutUser } = authSlice.actions;
export default authSlice.reducer;
