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
import {useAppDispatch} from "@/store/hooks.ts";
import {create, get, select_list, update} from "@/components/inventory/slice.ts";
import {type SelectOptionList,} from "@/lib/interfaces/common.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select"
import {useSelectList} from "@/hooks/use_select_list.ts";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useNavigate} from "react-router-dom";
import {useParams} from "react-router";

export default function Edit() {
  const [productId, setProductId] = React.useState("");
  const [warehouseId, setWarehouseId] = React.useState("");
  const [quantity, setQuantity] = React.useState("");
  const [cost, setCost] = React.useState("");
  const [price, setPrice] = React.useState("");
  const [currencyCode, setCurrencyCode] = React.useState("");
  const [currencyList, setCurrencyList] = React.useState<SelectOptionList>([]);
  const [productList, setProductList] = React.useState<SelectOptionList>([]);
  const [warehouseList, setWarehouseList] = React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();
  const {errors, setErrors, unexpectedError} = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      productId,
      warehouseId,
      quantity,
      cost,
      price,
      currencyCode,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/leltar/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [cost, currencyCode, dispatch, id, navigate, price, productId, quantity, setErrors, unexpectedError, warehouseId]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      productId,
      warehouseId,
      quantity,
      cost,
      price,
      currencyCode,
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/leltar/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [cost, currencyCode, dispatch, id, navigate, price, productId, quantity, setErrors, unexpectedError, warehouseId]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setProductId(data.product_id);
              setWarehouseId(data.warehouse_id);
              setQuantity(data.quantity.toString());
              setCost(data.cost ?? "");
              setPrice(data.price ?? "");
              setCurrencyCode(data.currency_code);
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
  }, [dispatch, id, setErrors, unexpectedError]);

  useEffect(() => {
    dispatch(select_list("currencies")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setCurrencyList, setErrors);
      } else {
        unexpectedError();
      }
    });
    dispatch(select_list("products")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setProductList, setErrors);
      } else {
        unexpectedError();
      }
    });
    dispatch(select_list("warehouses")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setWarehouseList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [
    dispatch,
    setErrors,
    unexpectedError,
    setListResponse
  ]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (typeof id === "string") {
      handleUpdate();
    } else {
      handleCreate();
    }
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
              return <SelectItem key={product.value} value={product.value}>{product.title}</SelectItem>
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
              return <SelectItem key={warehouse.value} value={warehouse.value}>{warehouse.title}</SelectItem>
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
        <Label htmlFor="currency_code">Pénznem</Label>
        <Select
          value={currencyCode}
          onValueChange={val => setCurrencyCode(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {currencyList.map(currency => {
              return <SelectItem key={currency.value} value={currency.value}>{currency.title}</SelectItem>
            })}
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"currency_code"}/>

        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}