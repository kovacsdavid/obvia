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
import {create, get, select_list, update} from "@/components/modules/taxes/lib/slice.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select.tsx"
import {useNavigate} from "react-router-dom";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useParams} from "react-router";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {useSelectList} from "@/hooks/use_select_list.ts";
import type {SelectOptionList} from "@/lib/interfaces/common.ts";

export default function Edit() {
  const [rate, setRate] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [countryCode, setCountryCode] = React.useState("");
  const [taxCategory, setTaxCategory] = React.useState("");
  const [isRateApplicable, setIsRateApplicable] = React.useState("");
  const [legalText, setLegalText] = React.useState("");
  const [reportingCode, setReportingCode] = React.useState("");
  const [isDefault, setIsDefault] = React.useState<boolean>(false);
  const [status, setStatus] = React.useState("");
  const [countryList, setCountryList] = React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();
  const {errors, setErrors, unexpectedError} = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const evalIsRateApplicable = (value: string): boolean | null => {
    switch (value) {
      case "true":
        return true;
      case "false":
        return false;
      default:
        return null;
    }
  };

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      rate,
      description,
      countryCode,
      taxCategory,
      isRateApplicable: evalIsRateApplicable(isRateApplicable),
      legalText,
      reportingCode,
      isDefault,
      status,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/ado/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [countryCode, description, dispatch, id, isDefault, isRateApplicable, legalText, navigate, rate, reportingCode, setErrors, status, taxCategory, unexpectedError]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      rate,
      description,
      countryCode,
      taxCategory,
      isRateApplicable: evalIsRateApplicable(isRateApplicable),
      legalText,
      reportingCode,
      isDefault,
      status,
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/ado/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [countryCode, description, dispatch, id, isDefault, isRateApplicable, legalText, navigate, rate, reportingCode, setErrors, status, taxCategory, unexpectedError]);

  useEffect(() => {
    dispatch(select_list("countries")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setCountryList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setErrors, setListResponse, unexpectedError]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setRate(data.rate ?? '');
              setDescription(data.description);
              setCountryCode(data.country_code);
              setTaxCategory(data.tax_category);
              setIsRateApplicable(data.is_rate_applicable ? "true" : "false");
              setLegalText(data.legal_text ?? '');
              setReportingCode(data.reporting_code ?? '');
              setIsDefault(data.is_default);
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
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Adók</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
            <Label htmlFor="is_rate_applicable">Adókulcs alkalmazandó</Label>
            <Select
              value={isRateApplicable}
              onValueChange={val => setIsRateApplicable(val)}
            >
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={"true"}>igen</SelectItem>
                <SelectItem value={"false"}>nem</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"is_rate_applicable"}/>
            {isRateApplicable === "true" ? (
              <>
                <Label htmlFor="rate">Adókulcs (%)</Label>
                <Input
                  id="rate"
                  type="text"
                  value={rate}
                  onChange={e => setRate(e.target.value)}
                />
                <FieldError error={errors} field={"rate"}/>
              </>
            ) : null}

            <Label htmlFor="description">Megnevezés</Label>
            <Input
              id="description"
              type="text"
              value={description}
              onChange={e => setDescription(e.target.value)}
            />
            <FieldError error={errors} field={"description"}/>

            <Label htmlFor="country_code">Ország</Label>
            <Select
              value={countryCode}
              onValueChange={val => setCountryCode(val)}
            >
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {countryList.map(country => {
                  return <SelectItem key={country.value} value={country.value}>{country.title}</SelectItem>
                })}
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"country_code"}/>

            <Label htmlFor="tax_category">Adó kategória</Label>
            <Select
              value={taxCategory}
              onValueChange={val => setTaxCategory(val)}
            >
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="standard">Általános</SelectItem>
                <SelectItem value="reduced">Kedvezményes</SelectItem>
                <SelectItem value="zero">Nulla kulcsos</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"tax_category"}/>

            <Label htmlFor="legal_text">Jogi szöveg</Label>
            <Input
              id="legal_text"
              type="text"
              value={legalText}
              onChange={e => setLegalText(e.target.value)}
            />
            <FieldError error={errors} field={"legal_text"}/>

            <Label htmlFor="reporting_code">Jelentési kód</Label>
            <Input
              id="reporting_code"
              type="text"
              value={reportingCode}
              onChange={e => setReportingCode(e.target.value)}
            />
            <FieldError error={errors} field={"reporting_code"}/>

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
                <SelectItem value="draft">Vázlat</SelectItem>
              </SelectContent>
            </Select>
            <FieldError error={errors} field={"status"}/>
            <Button type="submit">Létrehozás</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}