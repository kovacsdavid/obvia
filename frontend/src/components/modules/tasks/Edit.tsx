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
import {create, get, select_list, update} from "@/components/modules/tasks/lib/slice.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select.tsx";
import {useNavigate} from "react-router-dom";
import {useSelectList} from "@/hooks/use_select_list.ts";
import {useFormError} from "@/hooks/use_form_error.ts";
import type {SelectOptionList} from "@/lib/interfaces/common.ts";
import {useParams} from "react-router";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {formatDateToYMDHMS} from "@/lib/utils.ts";
import type {TaskUserInput} from "./lib/interface";

export default function Edit() {
  const [worksheetId, setWorksheetId] = React.useState("");
  const [serviceId, setServiceId] = React.useState("");
  const [currencyCode, setCurrencyCode] = React.useState("HUF");
  const [price, setPrice] = React.useState<string>("");
  const [taxId, setTaxId] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const [priority, setPriority] = React.useState<string | null>("normal");
  const [dueDate, setDueDate] = React.useState<string | null>("");
  const [worksheetList, setWorksheetList] = React.useState<SelectOptionList>([]);
  const [serviceList, setServiceList] = React.useState<SelectOptionList>([]);
  const [taxList, setTaxList] = React.useState<SelectOptionList>([]);
  const [currencyList, setCurrencyList] = React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();
  const {errors, setErrors, unexpectedError} = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const prepareTaskInput = useCallback((): TaskUserInput => ({
    id,
    worksheetId,
    serviceId,
    currencyCode,
    price,
    taxId,
    status,
    priority,
    dueDate
  }), [id, worksheetId, serviceId, currencyCode, price, taxId, status, priority, dueDate]);

  const handleCreate = useCallback(() => {
    dispatch(create(prepareTaskInput())).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/feladat/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, navigate, prepareTaskInput, setErrors, unexpectedError]);

  const handleUpdate = useCallback(() => {
    dispatch(update(prepareTaskInput())).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/feladat/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, navigate, prepareTaskInput, setErrors, unexpectedError]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setWorksheetId(data.worksheet_id);
              setServiceId(data.service_id);
              setCurrencyCode(data.currency_code);
              setPrice(data.price ?? "");
              setTaxId(data.tax_id);
              setStatus(data.status);
              setPriority(data.priority);
              setDueDate(data.due_date ? formatDateToYMDHMS(data.due_date) : null);
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
    dispatch(select_list("worksheets")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setWorksheetList, setErrors);
      } else {
        unexpectedError();
      }
    });
    dispatch(select_list("services")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setServiceList, setErrors);
      } else {
        unexpectedError();
      }
    });
    dispatch(select_list("taxes")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setTaxList, setErrors);
      } else {
        unexpectedError();
      }
    });
    dispatch(select_list("currencies")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setCurrencyList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setListResponse, setErrors, unexpectedError]);

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
          <CardTitle>Feladat</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
            <Label htmlFor="worksheet_id">Munkalap</Label>
            <Select value={worksheetId} onValueChange={setWorksheetId}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {worksheetList.map((worksheet) => (
                  <SelectItem key={worksheet.value} value={worksheet.value}>{worksheet.title}</SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"worksheet_id"}/>

            <Label htmlFor="service_id">Szolgáltatás</Label>
            <Select value={serviceId} onValueChange={setServiceId}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {serviceList.map((service) => (
                  <SelectItem key={service.value} value={service.value}>{service.title}</SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"service_id"}/>

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

            <Label htmlFor="price">Ár</Label>
            <Input
              id="price"
              type="text"
              value={price ?? ""}
              onChange={e => setPrice(e.target.value)}
            />
            <FieldError error={errors} field={"price"}/>

            <Label htmlFor="tax_id">Adó</Label>
            <Select value={taxId} onValueChange={setTaxId}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {taxList.map((tax) => (
                  <SelectItem key={tax.value} value={tax.value}>{tax.title}</SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"tax_id"}/>

            <Label htmlFor="status">Státusz</Label>
            <Select value={status} onValueChange={setStatus}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="active">Aktív</SelectItem>
                <SelectItem value="inactive">Inaktív</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"status"}/>

            <Label htmlFor="priority">Prioritás</Label>
            <Select value={priority ?? ""} onValueChange={setPriority}>
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="low">Alacsony</SelectItem>
                <SelectItem value="normal">Normál</SelectItem>
                <SelectItem value="high">Magas</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"priority"}/>

            <Label htmlFor="due_date">Határidő</Label>
            <Input
              id="due_date"
              type="date"
              value={dueDate ?? ""}
              onChange={e => setDueDate(e.target.value)}
            />
            <FieldError error={errors} field={"due_date"}/>

            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}