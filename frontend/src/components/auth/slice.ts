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

import {createAsyncThunk, createSlice, type PayloadAction} from "@reduxjs/toolkit";
import * as authApi from "@/components/auth/service.ts";
import type {LoginRequest, RegisterRequest} from "@/components/auth/interface.ts";

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
    token: string | null;
    status: "idle" | "loading" | "succeeded" | "failed",
    isLoggedIn: boolean;
  },
  register: {
    status: "idle" | "loading" | "succeeded" | "failed",
  },
}

const initialState: AuthState = {
  login: {
    user: null,
    token: null,
    status: "idle",
    isLoggedIn: false,
  },
  register: {
    status: "idle",
  }
};

export const registerUserRequest = createAsyncThunk(
  "auth/registerUserRequest",
  async (userData: RegisterRequest) => {
    return await authApi.register(userData);
  }
);

export const loginUserRequest = createAsyncThunk(
  "auth/loginUserRequest",
  async (credentials: LoginRequest, {rejectWithValue}) => {
    try {
      return await authApi.login(credentials);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
);

interface LoginUser {
  token: string,
  user: User
}

const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    loginUser(state, action: PayloadAction<LoginUser>) {
      state.login.user = action.payload.user;
      state.login.token = action.payload.token;
      state.login.isLoggedIn = true;
    },
    logoutUser(state) {
      state.login.user = null;
      state.login.token = null;
      state.login.status = "idle";
      state.login.isLoggedIn = false;
    },
    updateToken(state, action) {
      state.login.token = action.payload;
    }
  },
  extraReducers: (builder) => {
    builder
      .addCase(loginUserRequest.pending, (state) => {
        state.login.status = "loading";
      })
      .addCase(loginUserRequest.fulfilled, (state) => {
        state.login.status = "succeeded";
      })
      .addCase(loginUserRequest.rejected, (state) => {
        state.login.status = "failed";
      });
    builder
      .addCase(registerUserRequest.pending, (state) => {
        state.register.status = "loading";
      })
      .addCase(registerUserRequest.fulfilled, (state) => {
        state.register.status = "succeeded";
      })
      .addCase(registerUserRequest.rejected, (state) => {
        state.register.status = "failed";
      });
  },
});

export const {logoutUser, updateToken, loginUser} = authSlice.actions;
export default authSlice.reducer;
