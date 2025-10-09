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
import {create, get, update} from "@/components/services/slice.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select"
import {useNavigate} from "react-router-dom";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useParams} from "react-router";

export default function Edit() {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [defaultPrice, setDefaultPrice] = React.useState("");
  const [defaultTaxId, setDefaultTaxId] = React.useState("");
  const [currencyId, setCurrencyId] = React.useState("");
  const [status, setStatus] = React.useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {errors, setErrors, unexpectedError} = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      name,
      description,
      defaultPrice,
      defaultTaxId,
      currencyId,
      status,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/szolgaltatas/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [currencyId, defaultPrice, defaultTaxId, description, dispatch, id, name, navigate, setErrors, status, unexpectedError]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      name,
      description,
      defaultPrice,
      defaultTaxId,
      currencyId,
      status,
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/szolgaltatas/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [currencyId, defaultPrice, defaultTaxId, description, dispatch, id, name, navigate, setErrors, status, unexpectedError]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setName(data.name);
              setDescription(data.description ?? "");
              setDefaultPrice(data.default_price ?? "");
              setDefaultTaxId(data.default_tax_id ?? "");
              setCurrencyId(data.currency_id ?? "");
              setStatus(data.status);
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
        <Label htmlFor="name">Megnevezés</Label>
        <Input
          id="name"
          type="text"
          value={name}
          onChange={e => setName(e.target.value)}
        />
        <FieldError error={errors} field={"name"}/>

        <Label htmlFor="description">Leírás</Label>
        <Input
          id="description"
          type="text"
          value={description}
          onChange={e => setDescription(e.target.value)}
        />
        <FieldError error={errors} field={"description"}/>

        <Label htmlFor="default_price">Alapértelmezett ár</Label>
        <Input
          id="default_price"
          type="text"
          value={defaultPrice}
          onChange={e => setDescription(e.target.value)}
        />
        <FieldError error={errors} field={"default_price"}/>

        <Label htmlFor="default_tax_id">Alapértelmezett adózás</Label>
        <Select
          value={defaultTaxId}
          onValueChange={val => setDefaultTaxId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="other">Egyéb</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"default_tax_id"}/>

        <Label htmlFor="currency_id">Alapértelmezett pénznem</Label>
        <Select
          value={defaultTaxId}
          onValueChange={val => setDefaultTaxId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="other">Egyéb</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"currency_id"}/>

        <Label htmlFor="status">Státusz</Label>
        <Select
          value={status}
          onValueChange={val => setStatus(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="active">Aktív</SelectItem>
            <SelectItem value="lead">Érdeklődő</SelectItem>
            <SelectItem value="prospect">Lehetséges vevő</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"status"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}