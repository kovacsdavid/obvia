/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { configureStore } from "@reduxjs/toolkit";
import { Provider } from "react-redux";
import { MemoryRouter, Route, Routes } from "react-router";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { AuthProvider } from "@/context/auth/AuthProvider";
import reducer from "./lib/slice";
import Login from "./Login";
import Register from "./Register";
import ForgottenPassword from "./ForgottenPassword";
import EmailVerification from "./EmailVerification";
import { toast } from "@/components/ui/toast-lib";
import { registerUserRequest } from "./lib/slice";

const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), {
        status,
        headers: { "Content-Type": "application/json" },
    });

function renderAuth(path: string) {
    const store = configureStore({ reducer: { auth: reducer } });
    render(
        <Provider store={store}>
            <MemoryRouter initialEntries={[path]}>
                <AuthProvider>
                    <Routes>
                        <Route path="/login" element={<Login />} />
                        <Route path="/register" element={<Register />} />
                        <Route path="/forgot" element={<ForgottenPassword />} />
                        <Route
                            path="/forgot/:id"
                            element={<ForgottenPassword />}
                        />
                        <Route
                            path="/verify/:id"
                            element={<EmailVerification />}
                        />
                        <Route
                            path="/bejelentkezes"
                            element={<div>Login destination</div>}
                        />
                        <Route
                            path="/vezerlopult"
                            element={<div>Dashboard destination</div>}
                        />
                    </Routes>
                </AuthProvider>
            </MemoryRouter>
        </Provider>,
    );
    return store;
}

describe("auth screens", () => {
    beforeEach(() => vi.spyOn(toast, "add").mockImplementation(() => "toast"));
    afterEach(() => {
        cleanup();
        vi.unstubAllGlobals();
        vi.restoreAllMocks();
    });

    it("shows the OTP input when login requires MFA", async () => {
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValue(
                    json({ error: { message: "totp-required" } }, 401),
                ),
        );
        renderAuth("/login");
        fireEvent.change(screen.getByLabelText("Email"), {
            target: { value: "user@example.com" },
        });
        fireEvent.change(screen.getByLabelText("Jelszó"), {
            target: { value: "password" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Bejelentkezés" }));
        expect(
            await screen.findByText("Kétlépcsős azonosító kód"),
        ).toBeInTheDocument();
        expect(toast.add).not.toHaveBeenCalled();
    });

    it("stores the session and navigates after an OTP challenge", async () => {
        const data = {
            token: "fresh",
            claims: {
                sub: "user",
                exp: 9999999999,
                iat: 1,
                nbf: 1,
                iss: "issuer",
                aud: "audience",
                jti: "id",
                loc: "hu",
                tz: "Europe/Budapest",
                family_id: null,
                active_tenant: null,
            },
            user: {
                id: "user",
                email: "user@example.com",
                first_name: null,
                last_name: null,
                status: "active",
                profile_picture_url: null,
                is_mfa_enabled: false,
            },
        };
        const fetchMock = vi
            .fn()
            .mockResolvedValueOnce(
                json({ error: { message: "totp-required" } }, 401),
            )
            .mockResolvedValueOnce(json({ data }));
        vi.stubGlobal("fetch", fetchMock);
        const store = renderAuth("/login");
        fireEvent.click(screen.getByRole("button", { name: "Bejelentkezés" }));
        const otp = await screen.findByLabelText("Kétlépcsős azonosító kód");
        fireEvent.change(otp, { target: { value: "123456" } });
        await waitFor(() =>
            expect(store.getState().auth.login.token).toBe("fresh"),
        );
        expect(JSON.parse(fetchMock.mock.calls[1][1].body).otp).toBe("123456");
        expect(toast.add).not.toHaveBeenCalled();
        expect(
            await screen.findByText("Dashboard destination"),
        ).toBeInTheDocument();
    });

    it("shows registration field validation and navigates after a valid response", async () => {
        const fields = {
            email: "Invalid email",
            first_name: null,
            last_name: null,
            password: null,
            password_confirm: null,
        };
        const fetchMock = vi
            .fn()
            .mockResolvedValueOnce(
                json({ error: { message: "Invalid", fields } }, 422),
            )
            .mockResolvedValueOnce(json({ data: { message: "ok" } }, 201));
        vi.stubGlobal("fetch", fetchMock);
        renderAuth("/register");
        fireEvent.click(screen.getByRole("button", { name: "Regisztráció" }));
        expect(await screen.findByText("Invalid email")).toBeInTheDocument();
        expect(toast.add).toHaveBeenCalledTimes(1);
        expect(toast.add).toHaveBeenCalledWith(
            expect.objectContaining({ type: "error", description: "Invalid" }),
        );
        fireEvent.click(screen.getByRole("button", { name: "Regisztráció" }));
        expect(
            await screen.findByText("Login destination"),
        ).toBeInTheDocument();
    });

    it("shows password reset field errors and a success response", async () => {
        const fields = {
            token: null,
            password: "Invalid password",
            password_confirm: null,
        };
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValueOnce(
                    json({ error: { message: "Invalid", fields } }, 422),
                )
                .mockResolvedValueOnce(json({ data: { message: "ok" } })),
        );
        renderAuth("/forgot/12345678-1234-1234-1234-123456789012");
        fireEvent.click(
            screen.getByRole("button", { name: "Jelszó megváltoztatása" }),
        );
        expect(await screen.findByText("Invalid password")).toBeInTheDocument();
        expect(toast.add).toHaveBeenCalledTimes(1);
        fireEvent.click(
            screen.getByRole("button", { name: "Jelszó megváltoztatása" }),
        );
        expect(
            await screen.findByText(
                "A jelszó megváltoztatása sikeresen megtörtént.",
            ),
        ).toBeInTheDocument();
    });

    it("shows forgotten password success", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(json({ data: { message: "ok" } })),
        );
        renderAuth("/forgot");
        fireEvent.change(screen.getByLabelText("E-mail"), {
            target: { value: "user@example.com" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Küldés" }));
        expect(
            await screen.findByText(
                "Ha a megadott e-mail cím helyes, a jelszó helyreállításához szükséges levél elküldésre került.",
            ),
        ).toBeInTheDocument();
    });

    it("shows verification failure for malformed success data", async () => {
        vi.stubGlobal("fetch", vi.fn().mockResolvedValue(json({})));
        renderAuth("/verify/token");
        await waitFor(() =>
            expect(
                screen.getByText("Az e-mail cím megerősítése sikertelen."),
            ).toBeInTheDocument(),
        );
        expect(toast.add).toHaveBeenCalledTimes(1);
    });

    it("shows verification success without an error notification", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(json({ data: { message: "ok" } })),
        );
        renderAuth("/verify/token");
        expect(
            await screen.findByText(
                "Az e-mail cím megerősítése sikeresen megtörtént.",
            ),
        ).toBeInTheDocument();
        expect(toast.add).not.toHaveBeenCalled();
    });

    it("shows typed login validation and clears the field on editing", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(
                json(
                    {
                        error: {
                            message: "Invalid login",
                            fields: { email: "Invalid email" },
                        },
                    },
                    422,
                ),
            ),
        );
        renderAuth("/login");
        fireEvent.click(screen.getByRole("button", { name: "Bejelentkezés" }));
        expect(await screen.findByText("Invalid email")).toBeInTheDocument();
        expect(screen.getByLabelText("Email")).toHaveAttribute(
            "aria-invalid",
            "true",
        );
        expect(toast.add).toHaveBeenCalledTimes(1);
        fireEvent.change(screen.getByLabelText("Email"), {
            target: { value: "new@example.com" },
        });
        expect(screen.queryByText("Invalid email")).not.toBeInTheDocument();
    });

    it("keeps OTP entry visible for an OTP field error", async () => {
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValueOnce(
                    json({ error: { message: "totp-required" } }, 401),
                )
                .mockResolvedValueOnce(
                    json(
                        {
                            error: {
                                message: "Invalid",
                                fields: { otp: "Invalid code" },
                            },
                        },
                        422,
                    ),
                ),
        );
        renderAuth("/login");
        fireEvent.click(screen.getByRole("button", { name: "Bejelentkezés" }));
        await screen.findByText("Kétlépcsős azonosító kód");
        fireEvent.change(screen.getByLabelText("Kétlépcsős azonosító kód"), {
            target: { value: "123456" },
        });
        expect(await screen.findByText("Invalid code")).toBeInTheDocument();
        expect(
            screen.getByLabelText("Kétlépcsős azonosító kód"),
        ).toBeInTheDocument();
        expect(toast.add).toHaveBeenCalledTimes(1);
    });

    it("shows forgotten password validation with a single notification", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(
                json(
                    {
                        error: {
                            message: "Invalid",
                            fields: { email: "Invalid address" },
                        },
                    },
                    422,
                ),
            ),
        );
        renderAuth("/forgot");
        fireEvent.click(screen.getByRole("button", { name: "Küldés" }));
        expect(await screen.findByText("Invalid address")).toBeInTheDocument();
        expect(toast.add).toHaveBeenCalledTimes(1);
    });

    it("shows reset token validation that is not attached to an input", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(
                json(
                    {
                        error: {
                            message: "Invalid",
                            fields: {
                                token: "Invalid token",
                                password: null,
                                password_confirm: null,
                            },
                        },
                    },
                    422,
                ),
            ),
        );
        renderAuth("/forgot/12345678-1234-1234-1234-123456789012");
        fireEvent.click(
            screen.getByRole("button", { name: "Jelszó megváltoztatása" }),
        );
        expect(await screen.findByText("Invalid token")).toBeInTheDocument();
        expect(toast.add).toHaveBeenCalledTimes(1);
    });

    it.each([
        ["/login", "Bejelentkezés"],
        ["/register", "Regisztráció"],
        ["/forgot", "Küldés"],
        [
            "/forgot/12345678-1234-1234-1234-123456789012",
            "Jelszó megváltoztatása",
        ],
    ])("notifies generic failures once on %s", async (path, button) => {
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValue(
                    json({ error: { message: "Server failed" } }, 500),
                ),
        );
        renderAuth(path);
        fireEvent.click(screen.getByRole("button", { name: button }));
        await waitFor(() => expect(toast.add).toHaveBeenCalledTimes(1));
        expect(toast.add).toHaveBeenCalledWith(
            expect.objectContaining({
                type: "error",
                description: "Server failed",
            }),
        );
        expect(screen.queryByText("Server failed")).not.toBeInTheDocument();
    });

    it("supports silent registration errors without changing the result", async () => {
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValue(
                    json({ error: { message: "Server failed" } }, 500),
                ),
        );
        const store = renderAuth("/register");
        const result = await store
            .dispatch(
                registerUserRequest({
                    firstName: "",
                    lastName: "",
                    email: "",
                    password: "",
                    passwordConfirm: "",
                    silent: true,
                }),
            )
            .unwrap();
        expect(result).toMatchObject({ kind: "http_error", status: 500 });
        expect(toast.add).not.toHaveBeenCalled();
    });

    it("notifies unexpected success statuses without navigating", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(json({ data: { message: "ok" } }, 200)),
        );
        renderAuth("/register");
        fireEvent.click(screen.getByRole("button", { name: "Regisztráció" }));
        await waitFor(() => expect(toast.add).toHaveBeenCalledTimes(1));
        expect(screen.queryByText("Login destination")).not.toBeInTheDocument();
    });
});
