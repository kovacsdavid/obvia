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
import {create, select_list} from "@/store/slices/products.ts";
import { type FormError } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {isUnitsOfMeasureListResponse, type UnitsOfMeasureList} from "@/lib/interfaces/products.ts";

export default function Create() {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [unitOfMeasureId, setUnitOfMeasureId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [newUnitOfMeasure, setNewUnitOfMeasure] = React.useState("");
  const [unitsOfMeasureList, setUnitsOfMeasureList] = React.useState<UnitsOfMeasureList>([]);
  const [status, setStatus] = React.useState("active");
  const [errors, setErrors] = useState<FormError | null>(null);
  const dispatch = useAppDispatch();

  const unexpectedError = () => {
    setErrors({
      global: "Váratlan hiba történt a feldolgozás során!",
      fields: {}
    });
  }

  useEffect(() => {
    dispatch(select_list("units_of_measure")).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200:
              if (
                isUnitsOfMeasureListResponse(responseData)
                && typeof responseData.data !== "undefined"
              ) {
                setUnitsOfMeasureList(responseData.data);
              } else {
                unexpectedError();
              }
              break;
            default:
              unexpectedError();
          }
        } catch {
          unexpectedError();
        }
      }
    });
  }, [dispatch]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      name,
      description,
      unitOfMeasureId,
      newUnitOfMeasure,
      status
    })).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              window.location.href = "/termek/lista";
              break;
            case 422:
              setErrors(responseData.error);
              break;
            default:
              unexpectedError();
          }
        } catch {
          unexpectedError();
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors} />
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
        <Label htmlFor="name">Név</Label>
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
        <Label htmlFor="unit_of_measure">Mértékegység</Label>
        <Select
          value={unitOfMeasureId}
          onValueChange={val => setUnitOfMeasureId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {unitsOfMeasureList.map(unit_of_measure => {
              return <SelectItem key={unit_of_measure.id} value={unit_of_measure.id}>{unit_of_measure.unit_of_measure}</SelectItem>
            })}
            <SelectItem value="other">Egyéb</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"unit_of_measure"}/>

        {unitOfMeasureId === "other" ? (
          <>
            <Label htmlFor="new_unit_of_measure">Egyéb mértékegység</Label>
            <Input
              id="new_unit_of_measure"
              type="text"
              value={newUnitOfMeasure}
              onChange={e => setNewUnitOfMeasure(e.target.value)}
            />
            <FieldError error={errors} field={"new_unit_of_measure"}/>
          </>
        ) : null}
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
            <SelectItem value="inactive">Inaktív</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"status"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}