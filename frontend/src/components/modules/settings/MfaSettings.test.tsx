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
    act,
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ComponentProps, ReactNode } from "react";
import MfaSettings from "./MfaSettings";

const mocks = vi.hoisted(() => {
    const thunk = (type: string) =>
        Object.assign((argument?: unknown) => ({ type, argument }), {
            fulfilled: {
                match: (action: { type: string }) => action.type === type,
            },
        });
    return {
        dispatch: vi.fn(),
        notify: vi.fn(),
        enabled: false,
        enable: thunk("enable"),
        verify: thunk("verify"),
        disable: thunk("disable"),
        status: (value: boolean) => ({ type: "status", value }),
    };
});

vi.mock("@/store/hooks.ts", () => ({
    useAppDispatch: () => mocks.dispatch,
    useAppSelector: (selector: (state: unknown) => unknown) =>
        selector({
            auth: {
                login: {
                    user: {
                        email: "test@example.com",
                        is_mfa_enabled: mocks.enabled,
                    },
                },
            },
        }),
}));
vi.mock("@/components/modules/users/lib/slice.ts", () => ({
    enableOtp: mocks.enable,
    verifyOtp: mocks.verify,
    disableOtp: mocks.disable,
}));
vi.mock("@/components/modules/auth/lib/slice", () => ({
    chMfaStatus: mocks.status,
}));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: mocks.notify }));
vi.mock("qrcode.react", () => ({
    QRCodeSVG: ({ value }: { value: string }) => (
        <div data-testid="qr">{value}</div>
    ),
}));
vi.mock("@/components/ui", () => ({
    Button: ({
        children,
        variant: _variant,
        ...props
    }: ComponentProps<"button"> & { variant?: string }) => (
        <button {...props}>{children}</button>
    ),
    FieldErrorV2: ({ error }: { error?: string | null }) =>
        error ? <div>{error}</div> : null,
}));
vi.mock("@/components/ui/dialog.tsx", () => ({
    Dialog: ({ open, children }: { open: boolean; children: ReactNode }) =>
        open ? <div role="dialog">{children}</div> : null,
    DialogContent: ({ children }: { children: ReactNode }) => (
        <div>{children}</div>
    ),
    DialogTitle: ({ children }: { children: ReactNode }) => <h2>{children}</h2>,
}));
vi.mock("@/components/ui/field", () => ({
    Field: ({ children }: { children: ReactNode }) => <div>{children}</div>,
    FieldLabel: (props: ComponentProps<"label">) => <label {...props} />,
}));
vi.mock("@/components/ui/input-otp", () => ({
    InputOTP: ({
        children: _children,
        onChange,
        pattern: _pattern,
        ...props
    }: {
        children: ReactNode;
        onChange: (value: string) => void;
        pattern?: string;
    } & Omit<ComponentProps<"input">, "onChange">) => (
        <input {...props} onChange={(event) => onChange(event.target.value)} />
    ),
    InputOTPGroup: ({ children }: { children: ReactNode }) => <>{children}</>,
    InputOTPSeparator: () => null,
    InputOTPSlot: () => null,
}));

beforeEach(() => {
    mocks.enabled = false;
    mocks.dispatch.mockReset();
    mocks.notify.mockReset();
});
afterEach(cleanup);

function respond(
    type: string,
    payload: { kind: string; [key: string]: unknown },
) {
    mocks.dispatch.mockImplementationOnce((action) => {
        if (payload.kind !== "success" && !action.argument?.silent) {
            mocks.notify(payload);
        }
        return Promise.resolve({ type, payload });
    });
}

describe("MFA settings", () => {
    it("only opens setup when a usable secret is returned", async () => {
        mocks.dispatch.mockResolvedValueOnce({
            type: "enable",
            payload: { kind: "success", status: 200, data: {} },
        });
        render(<MfaSettings />);
        fireEvent.click(
            screen.getByRole("button", {
                name: "Kétlépcsős azonosítás bekapcsolása",
            }),
        );
        await waitFor(() =>
            expect(mocks.notify).toHaveBeenCalledWith({
                kind: "invalid_response",
                status: 200,
            }),
        );
        expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    });

    it("prevents repeated setup requests while one is pending", async () => {
        let resolveRequest!: (value: unknown) => void;
        mocks.dispatch.mockReturnValueOnce(
            new Promise((resolve) => {
                resolveRequest = resolve;
            }),
        );
        render(<MfaSettings />);
        const button = screen.getByRole("button", {
            name: "Kétlépcsős azonosítás bekapcsolása",
        });
        fireEvent.click(button);
        expect(button).toBeDisabled();
        fireEvent.click(button);
        expect(mocks.dispatch).toHaveBeenCalledTimes(1);
        await act(async () =>
            resolveRequest({
                type: "enable",
                payload: {
                    kind: "success",
                    status: 200,
                    data: { data: "SECRET" },
                },
            }),
        );
        expect(screen.getByTestId("qr")).toHaveTextContent("secret=SECRET");
    });

    it("shows OTP validation errors, clears them on input, and enables only on success", async () => {
        mocks.dispatch.mockResolvedValueOnce({
            type: "enable",
            payload: { kind: "success", status: 200, data: { data: "SECRET" } },
        });
        const failure = {
            kind: "http_error",
            status: 422,
            body: {
                error: {
                    message: "Invalid",
                    fields: { otp: "Wrong code" },
                },
            },
        };
        respond("verify", failure);
        mocks.dispatch.mockResolvedValueOnce({
            type: "verify",
            payload: {
                kind: "success",
                status: 200,
                data: { data: { message: "Done" } },
            },
        });
        render(<MfaSettings />);
        fireEvent.click(
            screen.getByRole("button", {
                name: "Kétlépcsős azonosítás bekapcsolása",
            }),
        );
        expect(await screen.findByTestId("qr")).toHaveTextContent(
            "secret=SECRET",
        );
        fireEvent.change(screen.getByLabelText("Megerősítő kód"), {
            target: { value: "123456" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Megerősítés" }));
        expect(await screen.findByText("Wrong code")).toBeInTheDocument();
        expect(screen.getByLabelText("Megerősítő kód")).toHaveAttribute(
            "aria-invalid",
            "true",
        );
        expect(screen.queryByText("Invalid")).not.toBeInTheDocument();
        expect(mocks.notify).toHaveBeenCalledOnce();
        expect(mocks.notify).toHaveBeenCalledWith(failure);
        expect(mocks.dispatch).toHaveBeenCalledWith({
            type: "verify",
            argument: { otp: "123456" },
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(mocks.status(true));
        fireEvent.change(screen.getByLabelText("Megerősítő kód"), {
            target: { value: "654321" },
        });
        expect(screen.queryByText("Wrong code")).not.toBeInTheDocument();
        expect(screen.getByLabelText("Megerősítő kód")).toHaveAttribute(
            "aria-invalid",
            "false",
        );
        fireEvent.click(screen.getByRole("button", { name: "Megerősítés" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(mocks.status(true)),
        );
        expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    });

    it("reports disable transport errors and changes status only after success", async () => {
        mocks.enabled = true;
        respond("disable", { kind: "timeout" });
        mocks.dispatch.mockResolvedValueOnce({
            type: "disable",
            payload: {
                kind: "success",
                status: 200,
                data: { data: { message: "Done" } },
            },
        });
        render(<MfaSettings />);
        fireEvent.click(
            screen.getByRole("button", {
                name: "Kétlépcsős azonosítás kikapcsolása",
            }),
        );
        fireEvent.change(screen.getByLabelText("Megerősítő kód"), {
            target: { value: "123456" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Megerősítés" }));
        await waitFor(() =>
            expect(mocks.notify).toHaveBeenCalledWith({ kind: "timeout" }),
        );
        expect(mocks.notify).toHaveBeenCalledOnce();
        expect(mocks.dispatch).toHaveBeenCalledWith({
            type: "disable",
            argument: { otp: "123456" },
        });
        expect(mocks.dispatch).not.toHaveBeenCalledWith(mocks.status(false));
        fireEvent.click(screen.getByRole("button", { name: "Megerősítés" }));
        await waitFor(() =>
            expect(mocks.dispatch).toHaveBeenCalledWith(mocks.status(false)),
        );
        expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    });

    it("clears OTP and errors when cancelling the disable dialog", async () => {
        mocks.enabled = true;
        const failure = {
            kind: "http_error",
            status: 401,
            body: {
                error: {
                    message: "Invalid code",
                    fields: { otp: "Wrong code" },
                },
            },
        };
        respond("disable", failure);
        render(<MfaSettings />);
        fireEvent.click(
            screen.getByRole("button", {
                name: "Kétlépcsős azonosítás kikapcsolása",
            }),
        );
        fireEvent.change(screen.getByLabelText("Megerősítő kód"), {
            target: { value: "123456" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Megerősítés" }));
        expect(await screen.findByText("Wrong code")).toBeInTheDocument();
        expect(screen.getByLabelText("Megerősítő kód")).toHaveAttribute(
            "aria-invalid",
            "true",
        );
        expect(mocks.notify).toHaveBeenCalledExactlyOnceWith(failure);
        fireEvent.change(screen.getByLabelText("Megerősítő kód"), {
            target: { value: "654321" },
        });
        expect(screen.queryByText("Wrong code")).not.toBeInTheDocument();
        expect(screen.getByLabelText("Megerősítő kód")).toHaveAttribute(
            "aria-invalid",
            "false",
        );
        respond("disable", failure);
        fireEvent.click(screen.getByRole("button", { name: "Megerősítés" }));
        expect(await screen.findByText("Wrong code")).toBeInTheDocument();
        fireEvent.click(screen.getByRole("button", { name: "Mégse" }));
        fireEvent.click(
            screen.getByRole("button", {
                name: "Kétlépcsős azonosítás kikapcsolása",
            }),
        );
        expect(screen.getByLabelText("Megerősítő kód")).toHaveValue("");
        expect(screen.queryByText("Wrong code")).not.toBeInTheDocument();
        expect(screen.getByLabelText("Megerősítő kód")).toHaveAttribute(
            "aria-invalid",
            "false",
        );
    });

    it("reports setup failures without opening the dialog", async () => {
        const failure = { kind: "http_error", status: 500 };
        respond("enable", failure);
        render(<MfaSettings />);
        fireEvent.click(
            screen.getByRole("button", {
                name: "Kétlépcsős azonosítás bekapcsolása",
            }),
        );
        await waitFor(() =>
            expect(mocks.notify).toHaveBeenCalledExactlyOnceWith(failure),
        );
        expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
        expect(mocks.dispatch).toHaveBeenCalledWith({
            type: "enable",
            argument: undefined,
        });
    });

    it.each([false, true])(
        "reports message-only OTP failures without changing status (enabled: %s)",
        async (enabled) => {
            mocks.enabled = enabled;
            if (!enabled) {
                respond("enable", {
                    kind: "success",
                    status: 200,
                    data: { data: "SECRET" },
                });
            }
            const failure = {
                kind: "http_error",
                status: 401,
                body: { error: { message: "Invalid code" } },
            };
            respond(enabled ? "disable" : "verify", failure);
            render(<MfaSettings />);
            fireEvent.click(
                screen.getByRole("button", {
                    name: enabled
                        ? "Kétlépcsős azonosítás kikapcsolása"
                        : "Kétlépcsős azonosítás bekapcsolása",
                }),
            );
            await screen.findByLabelText("Megerősítő kód");
            fireEvent.click(
                screen.getByRole("button", { name: "Megerősítés" }),
            );
            await waitFor(() =>
                expect(mocks.notify).toHaveBeenCalledExactlyOnceWith(failure),
            );
            expect(screen.queryByText("Invalid code")).not.toBeInTheDocument();
            expect(screen.getByLabelText("Megerősítő kód")).toHaveAttribute(
                "aria-invalid",
                "false",
            );
            expect(mocks.dispatch).not.toHaveBeenCalledWith(
                mocks.status(!enabled),
            );
            expect(screen.getByRole("dialog")).toBeInTheDocument();
        },
    );

    it("clears the setup secret and OTP when cancelling", async () => {
        respond("enable", {
            kind: "success",
            status: 200,
            data: { data: "FIRST" },
        });
        respond("enable", {
            kind: "success",
            status: 200,
            data: { data: "SECOND" },
        });
        render(<MfaSettings />);
        const enable = screen.getByRole("button", {
            name: "Kétlépcsős azonosítás bekapcsolása",
        });
        fireEvent.click(enable);
        expect(await screen.findByTestId("qr")).toHaveTextContent(
            "secret=FIRST",
        );
        fireEvent.click(
            screen.getByRole("button", { name: "Titkos kulcs megjelenítése" }),
        );
        expect(screen.getByText("FIRST")).toBeInTheDocument();
        fireEvent.change(screen.getByLabelText("Megerősítő kód"), {
            target: { value: "123456" },
        });
        fireEvent.click(screen.getByRole("button", { name: "Mégse" }));
        fireEvent.click(enable);
        expect(await screen.findByTestId("qr")).toHaveTextContent(
            "secret=SECOND",
        );
        expect(screen.getByLabelText("Megerősítő kód")).toHaveValue("");
        expect(screen.queryByText("FIRST")).not.toBeInTheDocument();
        expect(
            screen.getByRole("button", { name: "Titkos kulcs megjelenítése" }),
        ).toBeInTheDocument();
    });

    it.each([false, true])(
        "handles rejected setup requests (aborted: %s)",
        async (aborted) => {
            mocks.dispatch.mockResolvedValueOnce({
                type: "enable/rejected",
                meta: { aborted },
            });
            render(<MfaSettings />);
            const button = screen.getByRole("button", {
                name: "Kétlépcsős azonosítás bekapcsolása",
            });
            fireEvent.click(button);
            await waitFor(() => expect(button).toBeEnabled());
            if (aborted) {
                expect(mocks.notify).not.toHaveBeenCalled();
            } else {
                expect(mocks.notify).toHaveBeenCalledExactlyOnceWith({
                    kind: "network_error",
                    offline: false,
                });
            }
            expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
        },
    );

    it.each([false, true])(
        "protects pending OTP submissions and handles rejection (enabled: %s)",
        async (enabled) => {
            mocks.enabled = enabled;
            if (!enabled) {
                respond("enable", {
                    kind: "success",
                    status: 200,
                    data: { data: "SECRET" },
                });
            }
            let resolveRequest!: (value: unknown) => void;
            mocks.dispatch.mockReturnValueOnce(
                new Promise((resolve) => {
                    resolveRequest = resolve;
                }),
            );
            render(<MfaSettings />);
            fireEvent.click(
                screen.getByRole("button", {
                    name: enabled
                        ? "Kétlépcsős azonosítás kikapcsolása"
                        : "Kétlépcsős azonosítás bekapcsolása",
                }),
            );
            const input = await screen.findByLabelText("Megerősítő kód");
            fireEvent.change(input, { target: { value: "123456" } });
            const form = input.closest("form")!;
            fireEvent.submit(form);
            fireEvent.submit(form);
            expect(mocks.dispatch).toHaveBeenCalledTimes(enabled ? 1 : 2);
            expect(
                screen.getByRole("button", { name: "Megerősítés" }),
            ).toBeDisabled();
            expect(
                screen.getByRole("button", { name: "Mégse" }),
            ).toBeDisabled();
            await act(async () =>
                resolveRequest({ type: "rejected", meta: { aborted: false } }),
            );
            expect(mocks.notify).toHaveBeenCalledExactlyOnceWith({
                kind: "network_error",
                offline: false,
            });
            expect(mocks.dispatch).not.toHaveBeenCalledWith(
                mocks.status(!enabled),
            );
            expect(screen.getByRole("dialog")).toBeInTheDocument();
            expect(
                screen.getByRole("button", { name: "Megerősítés" }),
            ).toBeEnabled();
        },
    );
});
