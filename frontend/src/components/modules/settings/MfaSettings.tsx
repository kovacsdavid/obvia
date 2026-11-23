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

import React, { useCallback, useMemo } from "react";
import { Button, FieldErrorV2 } from "@/components/ui";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    enableOtp,
    verifyOtp,
    disableOtp,
} from "@/components/modules/users/lib/slice.ts";
import { QRCodeSVG } from "qrcode.react";
import { Eye, EyeClosed } from "lucide-react";
import type { OtpFields } from "@/components/modules/users/lib/interface.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { RootState } from "@/store";
import { useAppSelector } from "@/store/hooks.ts";
import { chMfaStatus } from "@/components/modules/auth/lib/slice";
import { Field, FieldLabel } from "@/components/ui/field";
import {
    InputOTP,
    InputOTPGroup,
    InputOTPSeparator,
    InputOTPSlot,
} from "@/components/ui/input-otp";
import { REGEXP_ONLY_DIGITS } from "input-otp";

export default function MfaSettings() {
    const dispatch = useAppDispatch();
    const [openVerifyOtpDialog, setOpenVerifyOtpDialog] = React.useState(false);
    const [openDisableOtpDialog, setOpenDisableOtpDialog] =
        React.useState(false);
    const [secret, setSecret] = React.useState("");
    const [showSecret, setShowSecret] = React.useState(false);
    const [otp, setOtp] = React.useState("");
    const [otpErrors, setOtpErrors] = React.useState<OtpFields | null>(null);
    const [pending, setPending] = React.useState(false);
    const pendingRef = React.useRef(false);
    const user = useAppSelector((state: RootState) => state.auth.login.user);
    const user_email = useMemo(() => user?.email, [user]);
    const user_is_mfa_enabled = useMemo(() => user?.is_mfa_enabled, [user]);

    const handleDisableMfa = useCallback(
        (e: React.SubmitEvent) => {
            e.preventDefault();
            if (pendingRef.current) return;
            pendingRef.current = true;
            setPending(true);
            setOtpErrors(null);
            dispatch(disableOtp({ otp }))
                .then((response) => {
                    if (disableOtp.fulfilled.match(response)) {
                        handleApiResult(response.payload, {
                            onSuccess: () => {
                                setOtp("");
                                setOpenDisableOtpDialog(false);
                                dispatch(chMfaStatus(false));
                            },
                            onFieldErrors: setOtpErrors,
                        });
                    } else if (!response.meta.aborted) {
                        notifyApiError({
                            kind: "network_error",
                            offline: false,
                        });
                    }
                })
                .finally(() => {
                    pendingRef.current = false;
                    setPending(false);
                });
        },
        [dispatch, otp],
    );

    const handleVerifyMfa = useCallback(
        (e: React.SubmitEvent) => {
            e.preventDefault();
            if (pendingRef.current) return;
            pendingRef.current = true;
            setPending(true);
            setOtpErrors(null);
            dispatch(verifyOtp({ otp }))
                .then((response) => {
                    if (verifyOtp.fulfilled.match(response)) {
                        handleApiResult(response.payload, {
                            onSuccess: () => {
                                setOtp("");
                                setOpenVerifyOtpDialog(false);
                                setSecret("");
                                setShowSecret(false);
                                dispatch(chMfaStatus(true));
                            },
                            onFieldErrors: setOtpErrors,
                        });
                    } else if (!response.meta.aborted) {
                        notifyApiError({
                            kind: "network_error",
                            offline: false,
                        });
                    }
                })
                .finally(() => {
                    pendingRef.current = false;
                    setPending(false);
                });
        },
        [dispatch, otp],
    );

    const handleEnableMfa = useCallback(() => {
        if (pendingRef.current) return;
        pendingRef.current = true;
        setPending(true);
        dispatch(enableOtp())
            .then((response) => {
                if (enableOtp.fulfilled.match(response)) {
                    handleApiResult(response.payload, {
                        onSuccess: (data) => {
                            if (
                                typeof data.data === "string" &&
                                data.data.length > 0
                            ) {
                                setSecret(data.data);
                                setOtp("");
                                setOtpErrors(null);
                                setOpenVerifyOtpDialog(true);
                            } else if (response.payload.kind === "success") {
                                notifyApiError({
                                    kind: "invalid_response",
                                    status: response.payload.status,
                                });
                            }
                        },
                    });
                } else if (!response.meta.aborted) {
                    notifyApiError({ kind: "network_error", offline: false });
                }
            })
            .finally(() => {
                pendingRef.current = false;
                setPending(false);
            });
    }, [dispatch]);

    return (
        <>
            <Dialog
                open={openDisableOtpDialog}
                onOpenChange={(open) => {
                    if (pendingRef.current) return;
                    setOpenDisableOtpDialog(open);
                    if (!open) {
                        setOtp("");
                        setOtpErrors(null);
                    }
                }}
            >
                <DialogContent className="text-center">
                    <DialogTitle>
                        Kétlépcsős azonosítás kikapcsolása
                    </DialogTitle>
                    <form
                        onSubmit={handleDisableMfa}
                        className="space-y-4"
                        autoComplete={"off"}
                    >
                        <Field className="w-fit mr-auto ml-auto mt-5 mb-10">
                            <FieldLabel htmlFor="otp">
                                Megerősítő kód
                            </FieldLabel>
                            <InputOTP
                                id="otp"
                                value={otp}
                                aria-invalid={!!otpErrors?.otp}
                                onChange={(value) => {
                                    setOtp(value);
                                    setOtpErrors(null);
                                }}
                                maxLength={6}
                                pattern={REGEXP_ONLY_DIGITS}
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
                            <FieldErrorV2 error={otpErrors?.otp} />
                        </Field>
                        <Button
                            className="mr-3"
                            variant="outline"
                            disabled={pending}
                            onClick={(e: React.MouseEvent) => {
                                e.preventDefault();
                                setOtp("");
                                setOpenDisableOtpDialog(false);
                                setOtpErrors(null);
                            }}
                        >
                            Mégse
                        </Button>
                        <Button type="submit" disabled={pending}>
                            Megerősítés
                        </Button>
                    </form>
                </DialogContent>
            </Dialog>
            <Dialog
                open={openVerifyOtpDialog}
                onOpenChange={(open) => {
                    if (pendingRef.current) return;
                    setOpenVerifyOtpDialog(open);
                    if (!open) {
                        setOtp("");
                        setOtpErrors(null);
                        setSecret("");
                        setShowSecret(false);
                    }
                }}
            >
                <DialogContent className="text-center">
                    <DialogTitle>
                        Kétlépcsős azonosítás bekapcsolása
                    </DialogTitle>
                    <div className="mr-auto ml-auto">
                        <QRCodeSVG
                            value={`otpauth://totp/obvia:${user_email}?secret=${secret}&issuer=obvia`}
                            level="H"
                        />
                    </div>
                    {showSecret ? (
                        <>
                            <Button
                                variant="outline"
                                onClick={() => setShowSecret(!showSecret)}
                            >
                                <EyeClosed /> Titkos kulcs elrejtése
                            </Button>
                            {secret}
                        </>
                    ) : (
                        <Button
                            variant="outline"
                            onClick={() => setShowSecret(!showSecret)}
                        >
                            <Eye /> Titkos kulcs megjelenítése
                        </Button>
                    )}
                    <form
                        onSubmit={handleVerifyMfa}
                        className="space-y-4"
                        autoComplete={"off"}
                    >
                        <Field className="w-fit mr-auto ml-auto mt-5 mb-10">
                            <FieldLabel htmlFor="otp">
                                Megerősítő kód
                            </FieldLabel>
                            <InputOTP
                                id="otp"
                                value={otp}
                                aria-invalid={!!otpErrors?.otp}
                                onChange={(value) => {
                                    setOtp(value);
                                    setOtpErrors(null);
                                }}
                                maxLength={6}
                                pattern={REGEXP_ONLY_DIGITS}
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
                            <FieldErrorV2 error={otpErrors?.otp} />
                        </Field>
                        <Button
                            className="mr-3"
                            variant="outline"
                            disabled={pending}
                            onClick={(e: React.MouseEvent) => {
                                e.preventDefault();
                                setOtp("");
                                setOpenVerifyOtpDialog(false);
                                setOtpErrors(null);
                                setSecret("");
                                setShowSecret(false);
                            }}
                        >
                            Mégse
                        </Button>
                        <Button type="submit" disabled={pending}>
                            Megerősítés
                        </Button>
                    </form>
                </DialogContent>
            </Dialog>
            {user_is_mfa_enabled ? (
                <Button
                    style={{ color: "red" }}
                    variant="outline"
                    disabled={pending}
                    onClick={() => setOpenDisableOtpDialog(true)}
                >
                    Kétlépcsős azonosítás kikapcsolása
                </Button>
            ) : (
                <Button
                    style={{ color: "green" }}
                    variant="outline"
                    disabled={pending}
                    onClick={() => handleEnableMfa()}
                >
                    Kétlépcsős azonosítás bekapcsolása
                </Button>
            )}
        </>
    );
}
