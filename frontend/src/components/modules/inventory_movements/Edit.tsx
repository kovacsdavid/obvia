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

import React, {useCallback, useEffect} from "react";
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {useAppDispatch} from "@/store/hooks.ts";
import {create, get, select_list} from "@/components/modules/inventory_movements/lib/slice.ts";
import {useNavigate, useParams} from "react-router-dom";
import {useFormError} from "@/hooks/use_form_error.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {type SelectOptionList} from "@/lib/interfaces/common.ts";
import {useSelectList} from "@/hooks/use_select_list.ts";

export default function Edit() {
  const [inventoryId, setInventoryId] = React.useState("");
  const [movementType, setMovementType] = React.useState("");
  const [quantity, setQuantity] = React.useState("");
  const [referenceType, setReferenceType] = React.useState<string>("");
  const [referenceId, setReferenceId] = React.useState<string>("");
  const [unitPrice, setUnitPrice] = React.useState<string>("");
  const [taxId, setTaxId] = React.useState("");
  const [taxList, setTaxList] = React.useState<SelectOptionList>([]);
  const [referenceIdList, setReferenceIdList] = React.useState<SelectOptionList>([])
  const [inventoryIdList, setInventoryIdList] = React.useState<SelectOptionList>([])
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const params = useParams();
  const {setListResponse} = useSelectList();
  const {errors, setErrors, unexpectedError} = useFormError();
  const routeInventoryId = React.useMemo(() => params["inventoryId"] ?? "", [params]);
  const id = React.useMemo(() => params["id"] ?? "", [params]);
  const totalPrice = React.useMemo(() => {
    const tpQuantity = parseFloat(quantity);
    const tpUnitPrice = parseFloat(unitPrice ?? "");

    if (!isNaN(tpQuantity) && !isNaN(tpUnitPrice)) {
      return (Math.abs(tpQuantity) * tpUnitPrice).toString();
    }
    return null;
  }, [quantity, unitPrice]);

  useEffect(() => {
    setInventoryId(routeInventoryId);
  }, [routeInventoryId]);

  const handleReferenceTypeChange = useCallback(async (newReferenceType: string) => {
    setReferenceType(newReferenceType);
    setReferenceIdList([]);
    return dispatch(select_list(newReferenceType)).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setReferenceIdList, setErrors);
        return response.payload;
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setErrors, setListResponse, unexpectedError]);

  const loadLists = useCallback(() => {
    return Promise.all(
      [!routeInventoryId && dispatch(select_list("inventory")).then(async (response) => {
        if (select_list.fulfilled.match(response)) {
          setListResponse(response.payload, setInventoryIdList, setErrors);
        } else {
          unexpectedError();
        }
      }),
        dispatch(select_list("taxes")).then(async (response) => {
          if (select_list.fulfilled.match(response)) {
            setListResponse(response.payload, setTaxList, setErrors);
          } else {
            unexpectedError();
          }
        })
      ]);
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
                setMovementType(data.movement_type);
                setQuantity(data.quantity ? data.quantity.toString() : "");
                handleReferenceTypeChange(data.reference_type ?? "").then(() => {
                  // TODO: there should be a better solution
                  setTimeout(() => {
                    setReferenceId(data.reference_id ?? "");
                  }, 0);
                });
                setUnitPrice(data.unit_price ?? "");
                setTaxId(data.tax_id);
              }
            } else if (typeof response.payload.jsonData?.error !== "undefined") {
              setErrors({message: response.payload.jsonData.error.message, fields: {}})
            } else {
              unexpectedError();
            }
          } else {
            unexpectedError();
          }
        });
      }
    });
  }, [dispatch, handleReferenceTypeChange, id, loadLists, setErrors, unexpectedError]);

  const handleSubmit = useCallback((e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      id: null,
      inventoryId,
      movementType,
      quantity,
      referenceType,
      referenceId,
      unitPrice,
      totalPrice,
      taxId,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate(-1);
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, inventoryId, movementType, quantity, referenceType, referenceId, unitPrice, totalPrice, taxId, navigate, setErrors, unexpectedError]);

  return (
    <>
      <GlobalError error={errors}/>
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Készletmozgás rögzítése</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
            {!routeInventoryId && (
              <>
                <Label htmlFor="inventoryId">Leltár azonosító</Label>
                <Select disabled={inventoryIdList.length === 0} value={inventoryId ?? ""}
                        onValueChange={val => setInventoryId(val)}>
                  <SelectTrigger className={"w-full"}>
                    <SelectValue/>
                  </SelectTrigger>
                  <SelectContent>
                    {inventoryIdList.map(inventoryIdListItem => (
                      <SelectItem key={inventoryIdListItem.value}
                                  value={inventoryIdListItem.value}>{inventoryIdListItem.title}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"reference_id"}/>
              </>
            )}

            <Label htmlFor="referenceType">Hivatkozás típusa</Label>
            <Select value={referenceType ?? ""} onValueChange={val => handleReferenceTypeChange(val)}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="worksheets">Munkalap</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"reference_type"}/>

            <Label htmlFor="referenceId">Hivatkozás azonosító</Label>
            <Select disabled={referenceIdList.length === 0} value={referenceId ?? ""}
                    onValueChange={val => setReferenceId(val)}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {referenceIdList.map(referenceIdListItem => (
                  <SelectItem key={referenceIdListItem.value}
                              value={referenceIdListItem.value}>{referenceIdListItem.title}</SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"reference_id"}/>

            <Label htmlFor="quantity">Mennyiség</Label>
            <Input
              id="quantity"
              type="number"
              value={quantity}
              onChange={e => setQuantity(e.target.value)}
            />
            <FieldError error={errors} field={"quantity"}/>

            <Label htmlFor="unitPrice">Egységár (nettó)</Label>
            <Input
              id="unitPrice"
              type="number"
              value={unitPrice ?? ""}
              onChange={e => setUnitPrice(e.target.value)}
            />
            <FieldError error={errors} field={"unit_price"}/>

            <Label htmlFor="totalPrice">Összesen (nettó)</Label>
            <Input
              id="totalPrice"
              type="number"
              value={totalPrice ?? ""}
              disabled={true}
            />

            <FieldError error={errors} field={"total_price"}/>
            <Label htmlFor="taxId">Adó</Label>
            <Select value={taxId} onValueChange={val => setTaxId(val)}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {taxList.map(tax => (
                  <SelectItem key={tax.value} value={tax.value}>{tax.title}</SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"tax_id"}/>

            <Label htmlFor="movementType">Művelet</Label>
            <Select value={movementType} onValueChange={val => setMovementType(val)}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="in">Bevétel</SelectItem>
                <SelectItem value="out">Kiadás</SelectItem>
                <SelectItem value="adjustment">Korrekció</SelectItem>
                <SelectItem value="transfer">Raktárak közötti mozgatás</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"movement_type"}/>

            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}
