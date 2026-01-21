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

import React, { useCallback } from "react";
import {
  Card,
  CardHeader,
  CardContent,
  CardTitle,
} from "@/components/ui/card.tsx";
import { Button } from "@/components/ui";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import { enableOtp } from "@/components/modules/auth/lib/slice.ts";
import { QRCodeSVG } from "qrcode.react";
import { Eye, EyeClosed } from "lucide-react";

export default function Settings() {
  const dispatch = useAppDispatch();
  const [openVerifyOtpDialog, setOpenVerifyOtpDialog] = React.useState(false);
  const [secret, setSecret] = React.useState("");
  const [showSecret, setShowSecret] = React.useState(false);

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
            <QRCodeSVG value={secret} level="H" />
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
        </DialogContent>
      </Dialog>
      <Card>
        <CardHeader>
          <CardTitle>Kétlépcsős azonosítás</CardTitle>
        </CardHeader>
        <CardContent>
          <Button
            style={{ color: "green" }}
            variant="outline"
            onClick={() => handleEnableMfa()}
          >
            Kétlépcsős azonosítás bekapcsolása
          </Button>
        </CardContent>
      </Card>
    </>
  );
}
