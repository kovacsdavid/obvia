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

import React, {useEffect, useState} from "react";
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create, select_list} from "@/store/slices/inventory.ts";
import { type FormError } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  type CurrencySelectListItem,
  isCurrencySelectListResponse, isProductSelectListResponse, isWarehouseSelectListResponse, type ProductSelectListItem,
  type WarehouseSelectListItem,
} from "@/services/inventory.ts";

export default function Create() {
  const [productId, setProductId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [warehouseId, setWarehouseId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [quantity, setQuantity] = React.useState("");
  const [cost, setCost] = React.useState("");
  const [price, setPrice] = React.useState("");
  const [currencyId, setCurrencyId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [newCurrency, setNewCurrecy] = React.useState("");
  const [currencyList, setCurrencyList] = React.useState<CurrencySelectListItem[]>([]);
  const [productList, setProductList] = React.useState<ProductSelectListItem[]>([]);
  const [warehouseList, setWarehouseList] = React.useState<WarehouseSelectListItem[]>([]);
  const [errors, setErrors] = useState<FormError | null>(null);
  const dispatch = useAppDispatch();

  useEffect(() => {
    dispatch(select_list("currencies")).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200:
              if (isCurrencySelectListResponse(responseData)) {
                setCurrencyList(responseData.data);
              } else {
                setErrors({
                  global: "Váratlan hiba történt a feldolgozás során!",
                  fields: {}
                });
              }
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
    dispatch(select_list("products")).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200:
              if (isProductSelectListResponse(responseData)) {
                setProductList(responseData.data);
              } else {
                setErrors({
                  global: "Váratlan hiba történt a feldolgozás során!",
                  fields: {}
                });
              }
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
    dispatch(select_list("warehouses")).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200:
              if (isWarehouseSelectListResponse(responseData)) {
                setWarehouseList(responseData.data);
              } else {
                setErrors({
                  global: "Váratlan hiba történt a feldolgozás során!",
                  fields: {}
                });
              }
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      productId,
      warehouseId,
      quantity,
      cost,
      price,
      currencyId,
      newCurrency,
    })).then(async (response) => {
      console.log(response)
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              window.location.href = "/leltar/lista";
              break;
            case 422:
              setErrors(responseData.error);
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4" autoComplete={"off"}>
        <Label htmlFor="product_id">Termék</Label>
        <Select
          value={productId}
          onValueChange={val => setProductId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {productList.map(product => {
              return <SelectItem key={product.id} value={product.id}>{product.name}</SelectItem>
            })}
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"product_id"}/>

        <Label htmlFor="warehouse_id">Raktár</Label>
        <Select
          value={warehouseId}
          onValueChange={val => setWarehouseId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {warehouseList.map(warehouse => {
              return <SelectItem key={warehouse.id} value={warehouse.id}>{warehouse.name}</SelectItem>
            })}
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"warehouse_id"}/>
        <Label htmlFor="quantity">Mennyiség</Label>
        <Input
          id="quantity"
          type="text"
          value={quantity}
          onChange={e => setQuantity(e.target.value)}
        />
        <FieldError error={errors} field={"quantity"}/>
        <FieldError error={errors} field={"unit_of_measure"}/>
        <Label htmlFor="cost">Bekerülési költség</Label>
        <Input
          id="cost"
          type="text"
          value={cost}
          onChange={e => setCost(e.target.value)}
        />
        <FieldError error={errors} field={"cost"}/>
        <Label htmlFor="price">Fogyasztói ár</Label>
        <Input
          id="price"
          type="text"
          value={price}
          onChange={e => setPrice(e.target.value)}
        />
        <FieldError error={errors} field={"price"}/>
        <Label htmlFor="currency_id">Pénznem</Label>
        <Select
          value={currencyId}
          onValueChange={val => setCurrencyId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {currencyList.map(currency => {
              return <SelectItem key={currency.id} value={currency.id}>{currency.currency}</SelectItem>
            })}
            <SelectItem value="other">Egyéb</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"currency_id"}/>

        {currencyId === "other" ? (
          <>
            <Label htmlFor="new_currency">Új pénznem</Label>
            <Input
              id="new_currency"
              type="text"
              value={newCurrency}
              onChange={e => setNewCurrecy(e.target.value)}
            />
            <FieldError error={errors} field={"new_currency"}/>
          </>
        ) : null}

        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}