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

import React, {useState} from "react";
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create} from "@/store/slices/inventory.ts";
import { type ErrorContainerWithFields } from "@/lib/interfaces.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

export default function Create() {
  const [productId, setProductId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [warehouseId, setWarehouseId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [quantity, setQuantity] = React.useState("");
  const [errors, setErrors] = useState<ErrorContainerWithFields | null>(null);
  const dispatch = useAppDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      productId,
      warehouseId,
      quantity,
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
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
        <Label htmlFor="product_id">Termék ID</Label>
        <Select
          value={productId}
          onValueChange={val => setProductId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="239b22ad-5db9-4c9c-851b-ba76885c2dae">Termék 1</SelectItem>
            <SelectItem value="9f68f241-5063-4965-ac60-d0fd0a3147eb">Termék 2</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"product_id"}/>
        <Label htmlFor="warehouse_id">Raktár ID</Label>
        <Select
          value={warehouseId}
          onValueChange={val => setWarehouseId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="239b22ad-5db9-4c9c-851b-ba76885c2dae">Raktár 1</SelectItem>
            <SelectItem value="9f68f241-5063-4965-ac60-d0fd0a3147eb">Raktár 2</SelectItem>
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
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}