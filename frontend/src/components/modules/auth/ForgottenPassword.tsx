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

import React, { useState } from "react";
import {
    Button,
    FieldErrorV2,
    GlobalNotification,
    Input,
    Label,
} from "@/components/ui";
import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@/components/ui/card.tsx";
import { useAppDispatch } from "@/store/hooks";
import {
    forgottenPassword,
    newPassword,
} from "@/components/modules/auth/lib/slice.ts";
import { useParams } from "react-router";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import { toast } from "@/components/ui/toast-lib";
import type {
    ForgottenPasswordFields,
    NewPasswordFields,
} from "@/components/modules/auth/lib/interface";

export default function ForgottenPassword() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [passwordConfirm, setPasswordConfirm] = useState("");
    const [forgottenErrors, setForgottenErrors] =
        useState<ForgottenPasswordFields | null>(null);
    const [newPasswordErrors, setNewPasswordErrors] =
        useState<NewPasswordFields | null>(null);
    const dispatch = useAppDispatch();
    const params = useParams();

    const handleSubmitNewPassword = async (e: React.SubmitEvent) => {
        e.preventDefault();
        const token = params["id"];
        if (typeof token === "string" && token.length === 36) {
            const response = await dispatch(
                newPassword({
                    token,
                    password,
                    password_confirm: passwordConfirm,
                }),
            );
            if (newPassword.fulfilled.match(response)) {
                const result = response.payload;
                setNewPasswordErrors(null);
                handleApiResult(result, {
                    onSuccess: () => {
                        if (
                            result.kind === "success" &&
                            result.status === 200
                        ) {
                            toast.add({
                                type: "success",
                                title: "A jelszó megváltoztatása sikeresen megtörtént.",
                            });
                        }
                    },
                    onFieldErrors: setNewPasswordErrors,
                });
            } else {
                if (!response.meta.aborted)
                    notifyApiError({ kind: "network_error", offline: false });
            }
        }
    };

    const handleSubmitForgottenPassword = async (e: React.SubmitEvent) => {
        e.preventDefault();

        const response = await dispatch(
            forgottenPassword({
                email,
            }),
        );
        if (forgottenPassword.fulfilled.match(response)) {
            const result = response.payload;
            setForgottenErrors(null);
            handleApiResult(result, {
                onSuccess: () => {
                    if (result.kind === "success" && result.status === 200) {
                        toast.add({
                            type: "success",
                            title: "Ha a megadott e-mail cím helyes, a jelszó helyreállításához szükséges levél elküldésre került.",
                        });
                    }
                },
                onFieldErrors: setForgottenErrors,
            });
        } else {
            if (!response.meta.aborted)
                notifyApiError({ kind: "network_error", offline: false });
        }
    };

    if (typeof params["id"] === "string" && params["id"].length === 36) {
        return (
            <>
                <Card className={"max-w-lg mx-auto"}>
                    <CardHeader>
                        <CardTitle>Add meg az új jelszót!</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <GlobalNotification message="Ezt az űrlapot csak akkor töltsd ki, ha te kértél jelszóemlékeztetőt!" />
                        <form
                            onSubmit={handleSubmitNewPassword}
                            className="space-y-4"
                            autoComplete={"off"}
                        >
                            <FieldErrorV2 error={newPasswordErrors?.token} />
                            <Label htmlFor="password">Jelszó</Label>
                            <Input
                                id="password"
                                type="password"
                                autoComplete="new-password"
                                value={password}
                                onChange={(e) => {
                                    setNewPasswordErrors((previous) =>
                                        previous
                                            ? { ...previous, password: null }
                                            : null,
                                    );
                                    setPassword(e.target.value);
                                }}
                                aria-invalid={!!newPasswordErrors?.password}
                            />
                            <FieldErrorV2 error={newPasswordErrors?.password} />
                            <Label htmlFor="password_confirm">
                                Jelszó megerősítése
                            </Label>
                            <Input
                                id="password_confirm"
                                type="password"
                                autoComplete="new-password"
                                value={passwordConfirm}
                                onChange={(e) => {
                                    setNewPasswordErrors((previous) =>
                                        previous
                                            ? {
                                                  ...previous,
                                                  password_confirm: null,
                                              }
                                            : null,
                                    );
                                    setPasswordConfirm(e.target.value);
                                }}
                                aria-invalid={
                                    !!newPasswordErrors?.password_confirm
                                }
                            />
                            <FieldErrorV2
                                error={newPasswordErrors?.password_confirm}
                            />
                            <Button type="submit">
                                Jelszó megváltoztatása
                            </Button>
                        </form>
                    </CardContent>
                </Card>
            </>
        );
    } else {
        return (
            <>
                <Card className={"max-w-lg mx-auto"}>
                    <CardHeader>
                        <CardTitle>Elfelejtett jelszó</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <GlobalNotification message="A megadott e-mail címre kapsz majd egy hivatkozást, aminek a segítségével megváltoztathatod a jelenlegi jelszavadat." />
                        <form
                            onSubmit={handleSubmitForgottenPassword}
                            className="space-y-4"
                            autoComplete={"off"}
                        >
                            <Label htmlFor="email">E-mail</Label>
                            <Input
                                id="email"
                                type="text"
                                autoComplete="email"
                                value={email}
                                onChange={(e) => {
                                    setForgottenErrors((previous) =>
                                        previous
                                            ? { ...previous, email: null }
                                            : null,
                                    );
                                    setEmail(e.target.value);
                                }}
                                aria-invalid={!!forgottenErrors?.email}
                            />
                            <FieldErrorV2 error={forgottenErrors?.email} />
                            <Button type="submit">Küldés</Button>
                        </form>
                    </CardContent>
                </Card>
            </>
        );
    }
}
