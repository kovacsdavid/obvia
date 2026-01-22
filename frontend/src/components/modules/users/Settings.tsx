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
import {
  Card,
  CardHeader,
  CardContent,
  CardTitle,
} from "@/components/ui/card.tsx";
import { Button, GlobalError, Input, Label } from "@/components/ui";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import { enableOtp, verifyOtp } from "@/components/modules/users/lib/slice.ts";
import { QRCodeSVG } from "qrcode.react";
import { Eye, EyeClosed } from "lucide-react";
import type { SimpleError } from "@/lib/interfaces/common";
import type { RootState } from "@/store";
import { useAppSelector } from "@/store/hooks.ts";

export default function Settings() {
  const dispatch = useAppDispatch();
  const [openVerifyOtpDialog, setOpenVerifyOtpDialog] = React.useState(false);
  const [secret, setSecret] = React.useState("");
  const [showSecret, setShowSecret] = React.useState(false);
  const [otp, setOtp] = React.useState("");
  const [otpErrors, setOtpErrors] = React.useState<SimpleError | null>(null);
  const user = useAppSelector((state: RootState) => state.auth.login.user);
  const user_email = useMemo(() => user?.email, [user]);
  const user_is_mfa_enabled = useMemo(() => user?.is_mfa_enabled, [user]);

  const handleDisableMfa = useCallback(() => {
    console.log("not implemented yet!");
  }, []);

  const handleVerfiyMfa = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      dispatch(verifyOtp(otp)).then((response) => {
        if (verifyOtp.fulfilled.match(response)) {
          if (
            response.payload.statusCode === 200 &&
            typeof response.payload.jsonData?.data !== "undefined"
          ) {
            setOtp("");
            setOpenVerifyOtpDialog(false);
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setOtpErrors(response.payload.jsonData.error);
          }
        }
      });
    },
    [dispatch, otp],
  );

  const handleEnableMfa = useCallback(() => {
    dispatch(enableOtp()).then((response) => {
      if (enableOtp.fulfilled.match(response)) {
        if (
          response.payload.statusCode === 200 &&
          typeof response.payload.jsonData?.data !== "undefined"
        ) {
          setSecret(response.payload.jsonData.data);
        }
        setOpenVerifyOtpDialog(true);
      }
    });
  }, [dispatch]);

  return (
    <>
      <Dialog open={openVerifyOtpDialog} onOpenChange={setOpenVerifyOtpDialog}>
        <DialogContent className="text-center">
          <DialogTitle>Kétlépcsős azonosítás bekapcsolása</DialogTitle>
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
            onSubmit={handleVerfiyMfa}
            className="space-y-4"
            autoComplete={"off"}
          >
            <GlobalError error={otpErrors} />
            <Label htmlFor="otp">Megerősítő kód</Label>
            <Input
              id="otp"
              type="text"
              value={otp}
              onChange={(e) => setOtp(e.target.value)}
            />
            <Button
              className="mr-3"
              variant="outline"
              onClick={(e: React.FormEvent) => {
                e.preventDefault();
                setOtp("");
                setOpenVerifyOtpDialog(false);
                setOtpErrors(null);
              }}
            >
              Mégse
            </Button>
            <Button type="submit">Megerősítés</Button>
          </form>
        </DialogContent>
      </Dialog>
      <Card>
        <CardHeader>
          <CardTitle>Kétlépcsős azonosítás</CardTitle>
        </CardHeader>
        <CardContent>
          {user_is_mfa_enabled ? (
            <Button
              style={{ color: "red" }}
              variant="outline"
              onClick={() => handleDisableMfa()}
            >
              Kétlépcsős azonosítás kikapcsolása
            </Button>
          ) : (
            <Button
              style={{ color: "green" }}
              variant="outline"
              onClick={() => handleEnableMfa()}
            >
              Kétlépcsős azonosítás bekapcsolása
            </Button>
          )}
        </CardContent>
      </Card>
    </>
  );
}
