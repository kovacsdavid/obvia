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

import React, { useCallback, useEffect } from "react";
import { Button, FieldError, GlobalError, Input, Label } from "@/components/ui";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import {
  create,
  get,
  select_list,
} from "@/components/modules/inventory_reservations/lib/slice.ts";
import { useNavigate, useParams } from "react-router-dom";
import { useFormError } from "@/hooks/use_form_error.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { type SelectOptionList } from "@/lib/interfaces/common.ts";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import InventoryEdit from "@/components/modules/inventory/Edit.tsx";
import type { Inventory } from "../inventory/lib/interface";
import { Plus } from "lucide-react";
import { useNumberInput } from "@/hooks/use_number_input.ts";

export default function Edit() {
  const [inventoryId, setInventoryId] = React.useState("");
  const [referenceType, setReferenceType] = React.useState<string | null>("");
  const [referenceId, setReferenceId] = React.useState<string | null>("");
  const [reservedUntil, setReservedUntil] = React.useState<string | null>(null);
  const [status, setStatus] = React.useState("");
  const [inventoryIdList, setInventoryIdList] =
    React.useState<SelectOptionList>([]);
  const [referenceIdList, setReferenceIdList] =
    React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const params = useParams();
  const { setListResponse } = useSelectList();
  const { errors, setErrors, unexpectedError } = useFormError();
  const routeInventoryId = React.useMemo(
    () => params["inventoryId"] ?? "",
    [params],
  );
  const id = React.useMemo(() => params["id"] ?? "", [params]);
  const [openNewInventoryDialog, setOpenNewInventoryDialog] =
    React.useState(false);
  const quantity = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 2,
    allowEmpty: true,
  });
  const handleEditInventorySuccess = (inventory: Inventory) => {
    loadLists().then(() => {
      setTimeout(() => {
        setInventoryId(inventory.id);
      }, 0);
      setOpenNewInventoryDialog(false);
    });
  };

  useEffect(() => {
    setInventoryId(routeInventoryId);
  }, [routeInventoryId]);

  const handleReferenceTypeChange = useCallback(
    async (newReferenceType: string) => {
      setReferenceType(newReferenceType);
      setReferenceIdList([]);
      return dispatch(select_list(newReferenceType)).then((response) => {
        if (select_list.fulfilled.match(response)) {
          setListResponse(response.payload, setReferenceIdList, setErrors);
        } else {
          unexpectedError();
        }
      });
    },
    [dispatch, setErrors, setListResponse, unexpectedError],
  );

  const loadLists = useCallback(async () => {
    if (!routeInventoryId) {
      return dispatch(select_list("inventory")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          setListResponse(response.payload, setInventoryIdList, setErrors);
        } else {
          unexpectedError();
        }
      });
    }
    return Promise.resolve();
  }, [dispatch, routeInventoryId, setErrors, setListResponse, unexpectedError]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string" && id.length === 36) {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setInventoryId(data.inventory_id);
                quantity.setValue(
                  data.quantity ? data.quantity.toString() : "",
                );
                handleReferenceTypeChange(data.reference_type ?? "").then(
                  () => {
                    setReferenceId(data.reference_id ?? "");
                  },
                );
                setReservedUntil(data.reserved_until);
                setStatus(data.status);
              }
            } else if (
              typeof response.payload.jsonData?.error !== "undefined"
            ) {
              setErrors({
                message: response.payload.jsonData.error.message,
                fields: {},
              });
            } else {
              unexpectedError();
            }
          } else {
            unexpectedError();
          }
        });
      }
    });
    // quantity is intentionally omitted to avoid infinite loops
    // They are only used to set initial values and don't need to trigger re-runs
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    dispatch,
    handleReferenceTypeChange,
    id,
    loadLists,
    setErrors,
    unexpectedError,
  ]);

  const handleSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      dispatch(
        create({
          id: id || null,
          inventoryId,
          quantity: !isNaN(quantity.getNumericValue())
            ? quantity.getNumericValue().toString()
            : "",
          referenceType,
          referenceId,
          reservedUntil,
          status,
        }),
      ).then(async (response) => {
        if (create.fulfilled.match(response)) {
          if (response.payload.statusCode === 201) {
            navigate(-1);
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setErrors(response.payload.jsonData.error);
          } else {
            unexpectedError();
          }
        } else {
          unexpectedError();
        }
      });
    },
    [
      dispatch,
      id,
      inventoryId,
      quantity,
      referenceType,
      referenceId,
      reservedUntil,
      status,
      navigate,
      setErrors,
      unexpectedError,
    ],
  );

  return (
    <>
      <GlobalError error={errors} />
      <Dialog
        open={openNewInventoryDialog}
        onOpenChange={setOpenNewInventoryDialog}
      >
        <DialogContent>
          <DialogTitle>Új munkalap létrehozása</DialogTitle>
          <InventoryEdit
            showCard={false}
            onSuccess={handleEditInventorySuccess}
          />
        </DialogContent>
      </Dialog>
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Készlet foglalás</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={handleSubmit}
            className="space-y-4"
            autoComplete={"off"}
          >
            {!routeInventoryId && (
              <>
                <Label htmlFor="inventoryId">Raktárkészlet</Label>
                <Select
                  disabled={inventoryIdList.length === 0}
                  value={inventoryId ?? ""}
                  onValueChange={(val) => setInventoryId(val)}
                >
                  <SelectTrigger className={"w-full"}>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {inventoryIdList.map((inventoryIdListItem) => (
                      <SelectItem
                        key={inventoryIdListItem.value}
                        value={inventoryIdListItem.value}
                      >
                        {inventoryIdListItem.title}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"inventory_id"} />
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => setOpenNewInventoryDialog(true)}
                >
                  <Plus /> Új raktárkészlet
                </Button>
              </>
            )}

            <Label htmlFor="quantity">Mennyiség</Label>
            <Input
              id="quantity"
              type="text"
              value={quantity.displayValue}
              onChange={(e) =>
                quantity.handleInputChangeWithCursor(e.target.value, e.target)
              }
            />
            <FieldError error={errors} field={"quantity"} />

            <Label htmlFor="referenceType">Hivatkozás típusa</Label>
            <Select
              value={referenceType ?? ""}
              onValueChange={(val) => handleReferenceTypeChange(val)}
            >
              <SelectTrigger className={"w-full"}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="worksheets">Munkalap</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"reference_type"} />

            <Label htmlFor="referenceId">Hivatkozás azonosító</Label>
            <Select
              disabled={referenceIdList.length === 0}
              value={referenceId ?? ""}
              onValueChange={(val) => setReferenceId(val)}
            >
              <SelectTrigger className={"w-full"}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {referenceIdList.map((referenceIdListItem) => (
                  <SelectItem
                    key={referenceIdListItem.value}
                    value={referenceIdListItem.value}
                  >
                    {referenceIdListItem.title}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"reference_id"} />

            <Label htmlFor="reservedUntil">Foglalás lejárata</Label>
            <Input
              id="reservedUntil"
              type="date"
              value={reservedUntil ?? ""}
              onChange={(e) => setReservedUntil(e.target.value)}
            />
            <FieldError error={errors} field={"reserved_until"} />

            <Label htmlFor="status">Státusz</Label>
            <Select value={status} onValueChange={(val) => setStatus(val)}>
              <SelectTrigger className={"w-full"}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="active">Aktív</SelectItem>
                <SelectItem value="fulfilled">Teljesített</SelectItem>
                <SelectItem value="cancelled">Lemondott</SelectItem>
                <SelectItem value="expired">Lejárt</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"status"} />

            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}
