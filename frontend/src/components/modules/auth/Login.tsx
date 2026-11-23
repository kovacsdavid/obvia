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

import React, { useState, useEffect } from "react";
import { useAppDispatch, useAppSelector } from "@/store/hooks.ts";
import type { RootState } from "@/store";
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { Link, useNavigate } from "react-router";
import { useAuth } from "@/hooks/use_auth";
import { loginUser } from "@/components/modules/auth/lib/slice.ts";
import { Card, CardContent } from "@/components/ui/card.tsx";
import { loginUserRequest } from "@/components/modules/auth/lib/slice.ts";
import type { LoginFields } from "@/components/modules/auth/lib/interface";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import {
    InputOTP,
    InputOTPGroup,
    InputOTPSeparator,
    InputOTPSlot,
} from "@/components/ui/input-otp";
import { REGEXP_ONLY_DIGITS } from "input-otp";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
    FieldSeparator,
} from "@/components/ui/field";

export default function Login() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [otp, setOtp] = useState("");
    const [otpRequired, setOtpRequired] = React.useState(false);
    const status = useAppSelector(
        (state: RootState) => state.auth.login.status,
    );
    const [errors, setErrors] = useState<LoginFields | null>(null);
    const resetError = (field: keyof LoginFields) =>
        setErrors((previous) =>
            previous ? { ...previous, [field]: null } : null,
        );
    const navigate = useNavigate();
    const { login, isLoggedIn } = useAuth();
    const dispatch = useAppDispatch();
    const loading = status === "loading";

    useEffect(() => {
        if (isLoggedIn) {
            navigate("/vezerlopult");
        }
    }, [isLoggedIn, navigate]);

    const handleSubmit = (
        e: React.SubmitEvent | null,
        nextOtp: string | null,
    ) => {
        if (e !== null) {
            e.preventDefault();
        }
        login(email, password, nextOtp).then(async (response) => {
            if (loginUserRequest.fulfilled.match(response)) {
                const result = response.payload;
                if (
                    result.kind === "http_error" &&
                    result.body?.error.message === "totp-required"
                ) {
                    setOtp("");
                    setErrors(null);
                    setOtpRequired(true);
                    return;
                }
                if (result.kind !== "success") {
                    setOtp("");
                    setOtpRequired(
                        result.kind === "http_error" &&
                            !!result.body?.error.fields?.otp,
                    );
                    setErrors(null);
                }
                handleApiResult(result, {
                    onSuccess: (data) => {
                        if (result.kind === "success" && result.status === 200)
                            dispatch(loginUser(data.data));
                    },
                    onFieldErrors: setErrors,
                });
            } else {
                setOtpRequired(false);
                setOtp("");
                if (!response.meta.aborted)
                    notifyApiError({ kind: "network_error", offline: false });
            }
        });
    };

    return (
        <>
            <Card className={"max-w-lg mx-auto"}>
                <CardContent>
                    <form
                        onSubmit={(e) => handleSubmit(e, null)}
                        className="space-y-4"
                        autoComplete={"off"}
                    >
                        <FieldSet>
                            <FieldLegend>Bejelentkezés</FieldLegend>
                            <FieldGroup>
                                <FieldLabel htmlFor="email">Email</FieldLabel>
                                <Input
                                    id="email"
                                    type="text"
                                    autoComplete="email"
                                    value={email}
                                    onChange={(e) => {
                                        resetError("email");
                                        setEmail(e.target.value);
                                    }}
                                    aria-invalid={!!errors?.email}
                                />
                                <FieldErrorV2 error={errors?.email} />
                                <FieldLabel htmlFor="password">
                                    Jelszó
                                </FieldLabel>
                                <Input
                                    id="password"
                                    type="password"
                                    autoComplete="current-password"
                                    value={password}
                                    onChange={(e) => {
                                        resetError("password");
                                        setPassword(e.target.value);
                                    }}
                                    aria-invalid={!!errors?.password}
                                />
                                <FieldErrorV2 error={errors?.password} />
                                {otpRequired && (
                                    <>
                                        <FieldSeparator />
                                        <Field className="">
                                            <FieldLabel htmlFor="otp">
                                                Kétlépcsős azonosító kód
                                            </FieldLabel>
                                            <InputOTP
                                                id="otp"
                                                value={otp}
                                                onChange={(e) => {
                                                    resetError("otp");
                                                    setOtp(e);
                                                    if (e.length === 6) {
                                                        handleSubmit(null, e);
                                                    }
                                                }}
                                                aria-invalid={!!errors?.otp}
                                                maxLength={6}
                                                pattern={REGEXP_ONLY_DIGITS}
                                                autoFocus
                                            >
                                                <InputOTPGroup>
                                                    <InputOTPSlot index={0} />
                                                    <InputOTPSlot index={1} />
                                                    <InputOTPSlot index={2} />
                                                    <InputOTPSeparator />
                                                    <InputOTPSlot index={3} />
                                                    <InputOTPSlot index={4} />
                                                    <InputOTPSlot index={5} />
                                                </InputOTPGroup>
                                            </InputOTP>
                                            <FieldErrorV2 error={errors?.otp} />
                                        </Field>
                                    </>
                                )}
                            </FieldGroup>
                        </FieldSet>
                        <Field orientation="horizontal">
                            <div className="text-right mt-8 w-full">
                                <Button type="submit" disabled={loading}>
                                    {otpRequired
                                        ? "Megerősítés"
                                        : loading
                                          ? "Bejelentkezés..."
                                          : "Bejelentkezés"}
                                </Button>
                            </div>
                        </Field>
                    </form>
                </CardContent>
            </Card>
            <div className="text-center mt-3">
                <Link to="/elfelejtett_jelszo">
                    <span className="text-xs">Elfelejtetted a jelszavad?</span>
                </Link>
            </div>
        </>
    );
}
