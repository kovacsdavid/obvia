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

import React from "react";
import {Button, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create} from "@/store/slices/inventory.ts";

export default function Create() {
  const [productId, setProductId] = React.useState("");
  const [warehouseId, setWarehouseId] = React.useState("");
  const [quantity, setQuantity] = React.useState("");
  const dispatch = useAppDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      productId,
      warehouseId,
      quantity,
    })).then((response) => {
      console.log(response)
    });
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
      <Label htmlFor="product_id">Termék ID</Label>
      <Input
        id="product_id"
        type="text"
        value={productId}
        onChange={e => setProductId(e.target.value)}
      />
      <Label htmlFor="warehouse_id">Raktár ID</Label>
      <Input
        id="warehouse_id"
        type="text"
        value={warehouseId}
        onChange={e => setWarehouseId(e.target.value)}
      />
      <Label htmlFor="quantity">Mennyiség</Label>
      <Input
        id="quantity"
        type="text"
        value={quantity}
        onChange={e => setQuantity(e.target.value)}
      />
      <Button type="submit">Létrehozás</Button>
    </form>
  );
}