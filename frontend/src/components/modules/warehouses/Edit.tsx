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
import {create, get, update} from "@/components/modules/warehouses/lib/slice.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select.tsx"
import {useNavigate} from "react-router-dom";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useParams} from "react-router";
import {ConditionalCard} from "@/components/ui/card.tsx";
import type {Warehouse} from "./lib/interface";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (warehouse: Warehouse) => void;
}

export default function List({showCard = true, onSuccess = undefined}: EditProps) {
  const [name, setName] = React.useState("");
  const [contactName, setContactName] = React.useState("");
  const [contactPhone, setContactPhone] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const {errors, setErrors, unexpectedError} = useFormError();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      name,
      contactName,
      contactPhone,
      status,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (
            typeof onSuccess === "function"
            && typeof response.payload.jsonData.data !== "undefined"
          ) {
            onSuccess(response.payload.jsonData.data);
          } else {
            navigate("/raktar/lista");
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [contactName, contactPhone, dispatch, id, name, navigate, onSuccess, setErrors, status, unexpectedError]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      name,
      contactName,
      contactPhone,
      status,
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/raktar/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [contactName, contactPhone, dispatch, id, name, navigate, setErrors, status, unexpectedError]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setName(data.name);
              setContactName(data.contact_name ?? "");
              setContactPhone(data.contact_phone ?? "");
              setStatus(data.status ?? "");
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
      <ConditionalCard
        showCard={showCard}
        title={`Raktár ${id ? "létrehozás" : "módosítás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
          <Label htmlFor="name">Név</Label>
          <Input
            id="name"
            type="text"
            value={name}
            onChange={e => setName(e.target.value)}
          />
          <FieldError error={errors} field={"name"}/>
          <Label htmlFor="contact_name">Kapcsolattartó neve</Label>
          <Input
            id="contact_name"
            type="text"
            value={contactName}
            onChange={e => setContactName(e.target.value)}
          />
          <FieldError error={errors} field={"contact_name"}/>
          <Label htmlFor="contact_phone">Kapcsolattartó telefonszáma</Label>
          <Input
            id="contact_phone"
            type="text"
            value={contactPhone}
            onChange={e => setContactPhone(e.target.value)}
          />
          <FieldError error={errors} field={"contact_phone"}/>
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
              <SelectItem value="inactive">Inaktív</SelectItem>
              <SelectItem value="maintenance">Karbantartás alatt</SelectItem>
              <SelectItem value="closed">Véglegesen bezárt</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"status"}/>
          <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
        </form>
      </ConditionalCard>
    </>
  );
}
