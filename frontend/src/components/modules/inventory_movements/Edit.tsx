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
import { Button, FieldError, GlobalError, Input } from "@/components/ui";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import {
  create,
  get,
  select_list,
} from "@/components/modules/inventory_movements/lib/slice.ts";
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
import type { InventoryMovement } from "./lib/interface";
import { useNumberInput } from "@/hooks/use_number_input.ts";
import { formatNumber, parseNumber } from "@/lib/utils.ts";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import InventoryEdit from "@/components/modules/inventory/Edit.tsx";
import type { Inventory } from "../inventory/lib/interface";
import { Plus } from "lucide-react";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (inventory_movement: InventoryMovement) => void;
  referenceType?: string | null;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
  referenceType = null,
}: EditProps) {
  const params = useParams();
  const referenceId = React.useMemo(() => params["referenceId"], [params]);
  const [inventoryId, setInventoryId] = React.useState("");
  const [movementType, setMovementType] = React.useState(
    typeof referenceId === "string" ? "out" : "",
  );
  const [taxId, setTaxId] = React.useState("");
  const [taxList, setTaxList] = React.useState<SelectOptionList>([]);
  React.useState<SelectOptionList>([]);
  const [inventoryIdList, setInventoryIdList] =
    React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { setListResponse } = useSelectList();
  const { errors, setErrors, unexpectedError, isInvalidField } = useFormError();
  const routeInventoryId = React.useMemo(
    () => params["inventoryId"] ?? "",
    [params],
  );
  const id = React.useMemo(() => params["id"] ?? "", [params]);
  const quantity = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 2,
    allowEmpty: true,
  });
  const unitPrice = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 2,
    allowEmpty: true,
  });
  const totalPrice = React.useMemo(() => {
    const tpQuantity = quantity.getNumericValue();
    const tpUnitPrice = unitPrice.getNumericValue();
    if (!isNaN(tpQuantity) && !isNaN(tpUnitPrice)) {
      return formatNumber(Math.abs(tpQuantity) * tpUnitPrice);
    }
    return null;
  }, [quantity, unitPrice]);

  useEffect(() => {
    setInventoryId(routeInventoryId);
  }, [routeInventoryId]);
  const [openNewInventoryDialog, setOpenNewInventoryDialog] =
    React.useState(false);

  const handleEditInventorySuccess = async (inventory: Inventory) => {
    return loadLists().then(() => {
      setTimeout(() => {
        setInventoryId(inventory.id);
      }, 0);
      setOpenNewInventoryDialog(false);
    });
  };

  const loadLists = useCallback(() => {
    return Promise.all([
      !routeInventoryId &&
        dispatch(select_list("inventory")).then((response) => {
          if (select_list.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              setListResponse(response.payload, setInventoryIdList, setErrors);
            } else {
              unexpectedError(response.payload.statusCode);
            }
          } else {
            unexpectedError();
          }
        }),
      dispatch(select_list("taxes")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setTaxList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
    ]);
  }, [dispatch, routeInventoryId, setErrors, setListResponse, unexpectedError]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string" && id.length === 36) {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData?.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setInventoryId(data.inventory_id);
                setMovementType(data.movement_type);
                quantity.setValue(
                  data.quantity ? data.quantity.toString() : "",
                );
                unitPrice.setValue(data.unit_price ?? "");
                setTaxId(data.tax_id);
              }
            } else if (
              typeof response.payload.jsonData?.error !== "undefined"
            ) {
              setErrors({
                message: response.payload.jsonData.error.message,
                fields: {},
              });
            } else {
              unexpectedError(response.payload.statusCode);
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
  }, [dispatch, id, loadLists, setErrors, unexpectedError]);

  const handleSubmit = useCallback(
    (e: React.SubmitEvent) => {
      e.preventDefault();
      dispatch(
        create({
          id: null,
          inventoryId,
          movementType,
          quantity: !isNaN(quantity.getNumericValue())
            ? quantity.getNumericValue().toString()
            : "",
          referenceType: typeof referenceType === "string" ? referenceType : "",
          referenceId: typeof referenceId === "string" ? referenceId : "",
          unitPrice: !isNaN(unitPrice.getNumericValue())
            ? unitPrice.getNumericValue().toString()
            : "",
          totalPrice:
            typeof totalPrice === "string"
              ? parseNumber(totalPrice).toString()
              : "",
          taxId,
        }),
      ).then(async (response) => {
        if (create.fulfilled.match(response)) {
          if (response.payload.statusCode === 201) {
            if (
              typeof onSuccess === "function" &&
              typeof response.payload.jsonData?.data !== "undefined"
            ) {
              onSuccess(response.payload.jsonData.data);
            } else {
              navigate(-1);
            }
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setErrors(response.payload.jsonData.error);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      });
    },
    [
      dispatch,
      inventoryId,
      movementType,
      quantity,
      referenceType,
      referenceId,
      unitPrice,
      totalPrice,
      taxId,
      onSuccess,
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
          <DialogTitle>Raktárkészlet létrehozása</DialogTitle>
          <InventoryEdit
            showCard={false}
            onSuccess={handleEditInventorySuccess}
            onCancel={() => setOpenNewInventoryDialog(false)}
          />
        </DialogContent>
      </Dialog>
      <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <FieldSet>
            <FieldLegend>
              {`Készletmozgás ${id ? "módosítás" : "létrehozás"}`}
            </FieldLegend>
            <FieldGroup>
              {!routeInventoryId && (
                <>
                  <Field data-invalid={isInvalidField(errors, "inventory_id")}>
                    <div className="flex items-center w-full">
                      <div className="flex flex-1 items-center">
                        <FieldLabel htmlFor="inventory_id">
                          Raktárkészlet
                        </FieldLabel>
                      </div>
                      <div className="flex items-center">
                        <Button
                          type="button"
                          variant="outline"
                          onClick={() => setOpenNewInventoryDialog(true)}
                        >
                          <Plus />
                        </Button>
                      </div>
                    </div>
                    <Select
                      disabled={inventoryIdList.length === 0}
                      value={inventoryId ?? ""}
                      onValueChange={(val) => setInventoryId(val)}
                    >
                      <SelectTrigger
                        className={"w-full"}
                        aria-invalid={isInvalidField(errors, "inventory_id")}
                      >
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
                  </Field>
                </>
              )}

              <Field data-invalid={isInvalidField(errors, "quantity")}>
                <FieldLabel htmlFor="quantity">Mennyiség</FieldLabel>
                <Input
                  id="quantity"
                  type="text"
                  placeholder="10"
                  value={quantity.displayValue}
                  onChange={(e) =>
                    quantity.handleInputChangeWithCursor(
                      e.target.value,
                      e.target,
                    )
                  }
                  aria-invalid={isInvalidField(errors, "quantity")}
                />
                <FieldError error={errors} field={"quantity"} />
              </Field>

              <Field data-invalid={isInvalidField(errors, "unit_price")}>
                <FieldLabel htmlFor="unit_price">Egységár (nettó)</FieldLabel>
                <Input
                  id="unit_price"
                  type="text"
                  placeholder="1 000,00"
                  value={unitPrice.displayValue}
                  onChange={(e) =>
                    unitPrice.handleInputChangeWithCursor(
                      e.target.value,
                      e.target,
                    )
                  }
                  aria-invalid={isInvalidField(errors, "unit_price")}
                />
                <FieldError error={errors} field={"unit_price"} />
              </Field>

              <Field data-invalid={isInvalidField(errors, "total_price")}>
                <FieldLabel htmlFor="total_price">Összesen (nettó)</FieldLabel>
                <Input
                  id="total_price"
                  type="text"
                  value={totalPrice ?? ""}
                  disabled={true}
                  aria-invalid={isInvalidField(errors, "name")}
                />
                <FieldError error={errors} field={"total_price"} />
              </Field>

              <Field data-invalid={isInvalidField(errors, "tax_id")}>
                <FieldLabel htmlFor="tax_id">Adó</FieldLabel>
                <Select value={taxId} onValueChange={(val) => setTaxId(val)}>
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField(errors, "tax_id")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {taxList.map((tax) => (
                      <SelectItem key={tax.value} value={tax.value}>
                        {tax.title}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"tax_id"} />
              </Field>
              <Field data-invalid={isInvalidField(errors, "movement_type")}>
                <FieldLabel htmlFor="movement_type">Művelet</FieldLabel>
                <Select
                  value={movementType}
                  onValueChange={(val) => setMovementType(val)}
                  disabled={typeof referenceId === "string"}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField(errors, "movement_type")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="in">Bevétel</SelectItem>
                    <SelectItem value="out">Kiadás</SelectItem>
                    {/*<SelectItem value="adjustment">Korrekció</SelectItem>*/}
                    {/*<SelectItem value="transfer">Raktárak közötti mozgatás</SelectItem>*/}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"movement_type"} />
              </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <div className="text-right mt-8 w-full">
              <Button
                className="mr-3"
                variant="outline"
                onClick={(e: React.MouseEvent) => {
                  e.preventDefault();
                  navigate(-1);
                }}
              >
                Mégse
              </Button>
              <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
            </div>
          </Field>
        </form>
      </ConditionalCard>
    </>
  );
}
