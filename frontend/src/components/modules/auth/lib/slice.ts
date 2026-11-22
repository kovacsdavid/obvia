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
    User,
    LoginData,
    LoginRequest,
    NewPasswordRequest,
    RegisterRequest,
} from "@/components/modules/auth/lib/interface.ts";
import type { RootState } from "@/store";
import type { NewTokenResponse } from "@/components/modules/databases/lib/interface.ts";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { ApiResult } from "@/lib/api/types";

function notifyAuthResult(
    result: ApiResult<unknown, unknown>,
    expectedStatus: number,
    silent = false,
) {
    if (silent) return;
    notifyApiError(
        result.kind === "success" && result.status !== expectedStatus
            ? { kind: "invalid_response", status: result.status }
            : result,
    );
}

interface AuthState {
    sessionExpired: boolean;
    sessionExpiryNotified: boolean;
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
    sessionExpired: false,
    sessionExpiryNotified: false,
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
    async (userData: RegisterRequest & { silent?: boolean }, { signal }) => {
        const result = await authApi.register(userData, signal);
        notifyAuthResult(result, 201, userData.silent);
        return result;
    },
);

export const loginUserRequest = createAsyncThunk(
    "auth/loginUserRequest",
    async (credentials: LoginRequest & { silent?: boolean }, { signal }) => {
        const result = await authApi.login(credentials, signal);
        if (!(
            result.kind === "http_error" &&
            result.body?.error.message === "totp-required"
        )) {
            notifyAuthResult(result, 200, credentials.silent);
        }
        return result;
    },
);

let logoutAndRevokeInFlight: Promise<void> | null = null;
let authGeneration = 0;

export function isLogoutInProgress(): boolean {
    return logoutAndRevokeInFlight !== null;
}

export const logoutAndRevokeRefreshToken = createAsyncThunk(
    "auth/logoutAndRevokeRefreshToken",
    async (options: { silent?: boolean } | void, { dispatch }) => {
        if (!logoutAndRevokeInFlight) {
            authGeneration += 1;
            logoutAndRevokeInFlight = (async () => {
                try {
                    if (refreshInFlight) await refreshInFlight;
                    const result = await authApi.logout();
                    if (!options?.silent) notifyApiError(result);
                } catch (error) {
                    if (!options?.silent)
                        notifyApiError({
                            kind: "network_error",
                            offline: false,
                        });
                    throw error;
                } finally {
                    logoutAndRevokeInFlight = null;
                    dispatch(logoutUser());
                }
            })();
        }
        await logoutAndRevokeInFlight;
    },
);

type RefreshFailure = Exclude<ApiResult<never>, { kind: "success" }>;
type RefreshOutcome = { kind: "success" } | RefreshFailure;
type SharedRefresh = { outcome: RefreshOutcome; notified: boolean };

let refreshInFlight: Promise<SharedRefresh> | null = null;

export const refreshAccessToken = createAsyncThunk(
    "auth/refreshAccessToken",
    async (
        options: { silent?: boolean } | void,
        { getState, dispatch },
    ): Promise<RefreshOutcome> => {
        if (logoutAndRevokeInFlight) {
            await logoutAndRevokeInFlight;
            return { kind: "aborted" };
        }
        const rootState = getState() as RootState;

        if (rootState.auth.sessionExpired) {
            const failure: RefreshFailure = { kind: "http_error", status: 401 };
            if (!options?.silent && !rootState.auth.sessionExpiryNotified) {
                dispatch(markSessionExpiryNotified());
                notifyApiError(failure);
            }
            return failure;
        }

        const token = rootState.auth.login.token;
        const expiresAt = rootState.auth.login.claims?.exp;

        const expiredOrNearExpiry =
            !token ||
            !expiresAt ||
            expiresAt - Math.floor(Date.now() / 1000) < 30;

        if (!expiredOrNearExpiry) return { kind: "success" };

        const generation = authGeneration;
        if (!refreshInFlight) {
            refreshInFlight = (async (): Promise<SharedRefresh> => {
                try {
                    const response = await authApi.refresh();
                    if (generation !== authGeneration)
                        return {
                            outcome: { kind: "aborted" },
                            notified: false,
                        };
                    if (
                        response.kind === "success" &&
                        response.status === 200
                    ) {
                        dispatch(loginUser(response.data.data));
                        return {
                            outcome: { kind: "success" },
                            notified: false,
                        };
                    }
                    const failure: RefreshFailure =
                        response.kind === "success"
                            ? {
                                  kind: "invalid_response",
                                  status: response.status,
                              }
                            : response.kind === "http_error" &&
                                response.status === 401
                              ? { kind: "http_error", status: 401 }
                              : response;
                    dispatch(logoutUser());
                    if (failure.kind === "http_error" && failure.status === 401)
                        dispatch(markSessionExpired());
                    return { outcome: failure, notified: false };
                } catch {
                    if (generation !== authGeneration)
                        return {
                            outcome: { kind: "aborted" },
                            notified: false,
                        };
                    dispatch(logoutUser());
                    return {
                        outcome: { kind: "network_error", offline: false },
                        notified: false,
                    };
                } finally {
                    refreshInFlight = null;
                }
            })();
        }
        const shared = await refreshInFlight;
        if (generation !== authGeneration) return { kind: "aborted" };
        if (
            shared.outcome.kind !== "success" &&
            !options?.silent &&
            !shared.notified
        ) {
            if (
                shared.outcome.kind === "http_error" &&
                shared.outcome.status === 401
            ) {
                if (!(getState() as RootState).auth.sessionExpiryNotified) {
                    dispatch(markSessionExpiryNotified());
                    notifyApiError(shared.outcome);
                }
            } else {
                notifyApiError(shared.outcome);
            }
            shared.notified = true;
        }
        return shared.outcome;
    },
);

export const get_claims = createAsyncThunk(
    "auth/get_claims",
    async (
        options: { silent?: boolean } | void,
        { getState, dispatch, signal },
    ) => {
        const refresh = await dispatch(refreshAccessToken(options));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" } as const;
        if (refresh.payload.kind !== "success") return refresh.payload;
        const rootState = getState() as RootState;
        const token = rootState.auth.login.token;
        if (!token || isLogoutInProgress()) return { kind: "aborted" } as const;
        const result = await authApi.get_claims(token, signal);
        if (!options?.silent) notifyApiError(result);
        return result;
    },
);

export const verfiy_email = createAsyncThunk(
    "auth/verfiy_email",
    async (input: string | { id: string; silent?: boolean }, { signal }) => {
        const id = typeof input === "string" ? input : input.id;
        const result = await authApi.verfiy_email(id, signal);
        notifyAuthResult(
            result,
            200,
            typeof input === "string" ? false : input.silent,
        );
        return result;
    },
);

export const forgottenPassword = createAsyncThunk(
    "auth/forgottenPassword",
    async (
        forgottenPasswordRequest: ForgottenPasswordRequest & {
            silent?: boolean;
        },
        { signal },
    ) => {
        const result = await authApi.forgottenPassword(
            forgottenPasswordRequest,
            signal,
        );
        notifyAuthResult(result, 200, forgottenPasswordRequest.silent);
        return result;
    },
);

export const newPassword = createAsyncThunk(
    "auth/newPassword",
    async (
        newPasswordRequest: NewPasswordRequest & { silent?: boolean },
        { signal },
    ) => {
        const result = await authApi.newPassword(newPasswordRequest, signal);
        notifyAuthResult(result, 200, newPasswordRequest.silent);
        return result;
    },
);

const authSlice = createSlice({
    name: "auth",
    initialState,
    reducers: {
        loginUser(state, action: PayloadAction<LoginData>) {
            state.sessionExpired = false;
            state.sessionExpiryNotified = false;
            state.login.claims = action.payload.claims;
            state.login.user = action.payload.user;
            state.login.token = action.payload.token;
            state.login.isLoggedIn = true;
            state.login.hasActiveDatabase =
                action.payload.claims.active_tenant !== null;
        },
        logoutUser(state) {
            state.sessionExpired = false;
            state.sessionExpiryNotified = false;
            state.login.claims = null;
            state.login.user = null;
            state.login.token = null;
            state.login.status = "idle";
            state.login.isLoggedIn = false;
            state.login.hasActiveDatabase = false;
        },
        markSessionExpired(state) {
            state.sessionExpired = true;
        },
        markSessionExpiryNotified(state) {
            state.sessionExpiryNotified = true;
        },
        updateToken(state, action: PayloadAction<NewTokenResponse>) {
            state.login.token = action.payload.token;
            state.login.claims = action.payload.claims;
            state.login.hasActiveDatabase =
                action.payload.claims.active_tenant !== null;
        },
        chMfaStatus(state, action: PayloadAction<boolean>) {
            if (state.login.user !== null) {
                state.login.user.is_mfa_enabled = action.payload;
            }
        },
    },
});

export const { logoutUser, updateToken, loginUser, chMfaStatus } =
    authSlice.actions;
const { markSessionExpired, markSessionExpiryNotified } = authSlice.actions;
export default authSlice.reducer;
