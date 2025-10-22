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
import {create, get, select_list, update} from "@/components/modules/products/lib/slice.ts";
import {type SelectOptionList} from "@/lib/interfaces/common.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select.tsx";
import {useNavigate} from "react-router-dom";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useSelectList} from "@/hooks/use_select_list.ts";
import {useParams} from "react-router";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";

export default function Edit() {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [unitOfMeasureId, setUnitOfMeasureId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [newUnitOfMeasure, setNewUnitOfMeasure] = React.useState("");
  const [unitsOfMeasureList, setUnitsOfMeasureList] = React.useState<SelectOptionList>([]);
  const [status, setStatus] = React.useState("active");
  const {errors, setErrors, unexpectedError} = useFormError();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      name,
      description,
      unitOfMeasureId,
      newUnitOfMeasure,
      status
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/termek/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [description, dispatch, id, name, navigate, newUnitOfMeasure, setErrors, status, unexpectedError, unitOfMeasureId]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      name,
      description,
      unitOfMeasureId,
      newUnitOfMeasure,
      status
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/termek/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [description, dispatch, id, name, navigate, newUnitOfMeasure, setErrors, status, unexpectedError, unitOfMeasureId]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setName(data.name);
              setDescription(data.description ?? "");
              setUnitOfMeasureId(data.unit_of_measure_id);
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

  useEffect(() => {
    dispatch(select_list("units_of_measure")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setUnitsOfMeasureList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setErrors, setListResponse, unexpectedError]);

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
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Termék</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
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
                  return <SelectItem key={unit_of_measure.value}
                                     value={unit_of_measure.value}>{unit_of_measure.title}</SelectItem>
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
            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}