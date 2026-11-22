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
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { registerUserRequest } from "@/components/modules/auth/lib/slice.ts";
import { useAppDispatch } from "@/store/hooks.ts";
import { useNavigate } from "react-router";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { RegisterFields } from "@/components/modules/auth/lib/interface";
import { Card, CardContent } from "@/components/ui/card.tsx";
import { useAuth } from "@/hooks/use_auth";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";

export default function Register() {
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [passwordConfirm, setPasswordConfirm] = useState("");
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const [errors, setErrors] = useState<RegisterFields | null>(null);
    const isInvalidField = (field: keyof RegisterFields) => !!errors?.[field];
    const resetError = (field: keyof RegisterFields) =>
        setErrors((previous) =>
            previous ? { ...previous, [field]: null } : null,
        );
    const { isLoggedIn } = useAuth();

    useEffect(() => {
        if (isLoggedIn) {
            navigate("/adatbazis/letrehozas");
        }
    }, [isLoggedIn, navigate]);

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();

        const response = await dispatch(
            registerUserRequest({
                firstName,
                lastName,
                email,
                password,
                passwordConfirm,
            }),
        );
        if (registerUserRequest.fulfilled.match(response)) {
            const result = response.payload;
            setErrors(null);
            handleApiResult(result, {
                onSuccess: () => {
                    if (result.kind === "success" && result.status === 201)
                        navigate("/bejelentkezes");
                },
                onFieldErrors: setErrors,
            });
        } else {
            if (!response.meta.aborted)
                notifyApiError({ kind: "network_error", offline: false });
        }
    };

    return (
        <>
            <Card className={"max-w-lg mx-auto"}>
                <CardContent>
                    <form
                        onSubmit={handleSubmit}
                        className="space-y-4"
                        autoComplete={"off"}
                    >
                        <FieldSet>
                            <FieldLegend>Regisztráció</FieldLegend>
                            <FieldGroup>
                                <Field
                                    data-invalid={isInvalidField("last_name")}
                                >
                                    <FieldLabel htmlFor="last_name">
                                        Vezetéknév
                                    </FieldLabel>
                                    <Input
                                        id="last_name"
                                        type="text"
                                        value={lastName}
                                        onChange={(e) => {
                                            resetError("last_name");
                                            setLastName(e.target.value);
                                        }}
                                        aria-invalid={isInvalidField(
                                            "last_name",
                                        )}
                                    />
                                    <FieldErrorV2 error={errors?.last_name} />
                                </Field>
                                <Field
                                    data-invalid={isInvalidField("first_name")}
                                >
                                    <FieldLabel htmlFor="first_name">
                                        Keresztnév
                                    </FieldLabel>
                                    <Input
                                        id="first_name"
                                        type="text"
                                        value={firstName}
                                        onChange={(e) => {
                                            resetError("first_name");
                                            setFirstName(e.target.value);
                                        }}
                                        aria-invalid={isInvalidField(
                                            "first_name",
                                        )}
                                    />
                                    <FieldErrorV2 error={errors?.first_name} />
                                </Field>
                                <Field data-invalid={isInvalidField("email")}>
                                    <FieldLabel htmlFor="email">
                                        Email
                                    </FieldLabel>
                                    <Input
                                        id="email"
                                        type="text"
                                        autoComplete="email"
                                        value={email}
                                        onChange={(e) => {
                                            resetError("email");
                                            setEmail(e.target.value);
                                        }}
                                        aria-invalid={isInvalidField("email")}
                                    />
                                    <FieldErrorV2 error={errors?.email} />
                                </Field>
                                <Field
                                    data-invalid={isInvalidField("password")}
                                >
                                    <FieldLabel htmlFor="password">
                                        Jelszó
                                    </FieldLabel>
                                    <Input
                                        id="password"
                                        type="password"
                                        autoComplete="new-password"
                                        value={password}
                                        onChange={(e) => {
                                            resetError("password");
                                            setPassword(e.target.value);
                                        }}
                                        aria-invalid={isInvalidField(
                                            "password",
                                        )}
                                    />
                                    <FieldErrorV2 error={errors?.password} />
                                </Field>
                                <Field
                                    data-invalid={isInvalidField(
                                        "password_confirm",
                                    )}
                                >
                                    <FieldLabel htmlFor="password_confirm">
                                        Jelszó megerősítése
                                    </FieldLabel>
                                    <Input
                                        id="password_confirm"
                                        type="password"
                                        autoComplete="new-password"
                                        value={passwordConfirm}
                                        onChange={(e) => {
                                            resetError("password_confirm");
                                            setPasswordConfirm(e.target.value);
                                        }}
                                        aria-invalid={isInvalidField(
                                            "password_confirm",
                                        )}
                                    />
                                    <FieldErrorV2
                                        error={errors?.password_confirm}
                                    />
                                </Field>
                            </FieldGroup>
                        </FieldSet>
                        <Field orientation="horizontal">
                            <div className="text-right mt-8 w-full">
                                <Button type="submit">Regisztráció</Button>
                            </div>
                        </Field>
                    </form>
                </CardContent>
            </Card>
        </>
    );
}
