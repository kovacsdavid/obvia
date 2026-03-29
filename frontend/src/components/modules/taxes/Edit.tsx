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

import React, { useCallback, useEffect } from "react";
import { Button, FieldError, GlobalError, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
  create,
  get,
  select_list,
  update,
} from "@/components/modules/taxes/lib/slice.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router-dom";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { useSelectList } from "@/hooks/use_select_list.ts";
import type { SelectOptionList } from "@/lib/interfaces/common.ts";
import type { Tax } from "./lib/interface";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (tax: Tax) => void;
  onCancel?: () => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
  onCancel = undefined,
}: EditProps) {
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
  const { setListResponse } = useSelectList();
  const { errors, setErrors, unexpectedError, isInvalidField, resetError } =
    useFormError();
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
    dispatch(
      create({
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
      }),
    ).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (
            typeof onSuccess === "function" &&
            typeof response.payload.jsonData?.data !== "undefined"
          ) {
            onSuccess(response.payload.jsonData.data);
          } else {
            navigate("/ado/lista");
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    countryCode,
    description,
    dispatch,
    id,
    isDefault,
    isRateApplicable,
    legalText,
    navigate,
    onSuccess,
    rate,
    reportingCode,
    setErrors,
    status,
    taxCategory,
    unexpectedError,
  ]);

  const handleCancel = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      if (typeof onCancel === "function") {
        onCancel();
      } else {
        navigate(-1);
      }
    },
    [navigate, onCancel],
  );

  const handleUpdate = useCallback(() => {
    dispatch(
      update({
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
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/ado/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    countryCode,
    description,
    dispatch,
    id,
    isDefault,
    isRateApplicable,
    legalText,
    navigate,
    rate,
    reportingCode,
    setErrors,
    status,
    taxCategory,
    unexpectedError,
  ]);

  const loadLists = useCallback(async () => {
    return dispatch(select_list("countries")).then((response) => {
      if (select_list.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          setListResponse(response.payload, setCountryList, setErrors);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setErrors, setListResponse, unexpectedError]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string") {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData?.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setRate(data.rate ?? "");
                setDescription(data.description);
                setCountryCode(data.country_code);
                setTaxCategory(data.tax_category);
                setIsRateApplicable(data.is_rate_applicable ? "true" : "false");
                setLegalText(data.legal_text ?? "");
                setReportingCode(data.reporting_code ?? "");
                setIsDefault(data.is_default);
                setStatus(data.status);
              }
            } else if (
              typeof response.payload.jsonData?.error !== "undefined"
            ) {
              setErrors({
                message: response.payload.jsonData.error.message,
                fields: {},
              });
            } else {
              unexpectedError(response.payload.statusCode);
            }
          } else {
            unexpectedError();
          }
        });
      }
    });
  }, [dispatch, id, setErrors, unexpectedError, setListResponse, loadLists]);

  const handleSubmit = async (e: React.SubmitEvent) => {
    e.preventDefault();
    if (typeof id === "string") {
      handleUpdate();
    } else {
      handleCreate();
    }
  };

  return (
    <>
      <GlobalError error={errors} />
      <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <FieldSet>
            <FieldLegend>
              {`Adók ${id ? "módosítás" : "létrehozás"}`}
            </FieldLegend>
            <FieldGroup>
              <Field data-invalid={isInvalidField("is_rate_applicable")}>
                <FieldLabel htmlFor="is_rate_applicable">
                  Adókulcs alkalmazandó
                </FieldLabel>
                <Select
                  value={isRateApplicable}
                  onValueChange={(val) => {
                    resetError("is_rate_applicable");
                    setIsRateApplicable(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("is_rate_applicable")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value={"true"}>igen</SelectItem>
                    <SelectItem value={"false"}>nem</SelectItem>
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"is_rate_applicable"} />
              </Field>
              {isRateApplicable === "true" ? (
                <>
                  <Field data-invalid={isInvalidField("rate")}>
                    <FieldLabel htmlFor="rate">Adókulcs (%)</FieldLabel>
                    <Input
                      id="rate"
                      type="text"
                      value={rate}
                      onChange={(e) => {
                        resetError("rate");
                        setRate(e.target.value);
                      }}
                      aria-invalid={isInvalidField("rate")}
                    />
                    <FieldError error={errors} field={"rate"} />
                  </Field>
                </>
              ) : null}
              <Field data-invalid={isInvalidField("description")}>
                <FieldLabel htmlFor="description">Megnevezés</FieldLabel>
                <Input
                  id="description"
                  type="text"
                  placeholder="ÁFA"
                  value={description}
                  onChange={(e) => {
                    resetError("description");
                    setDescription(e.target.value);
                  }}
                  aria-invalid={isInvalidField("description")}
                />
                <FieldError error={errors} field={"description"} />
              </Field>
              <Field data-invalid={isInvalidField("country_code")}>
                <FieldLabel htmlFor="country_code">Ország</FieldLabel>
                <Select
                  value={countryCode}
                  onValueChange={(val) => {
                    resetError("country_code");
                    setCountryCode(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("country_code")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {countryList.map((country) => {
                      return (
                        <SelectItem key={country.value} value={country.value}>
                          {country.title}
                        </SelectItem>
                      );
                    })}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"country_code"} />
              </Field>
              <Field data-invalid={isInvalidField("tax_category")}>
                <FieldLabel htmlFor="tax_category">Adó kategória</FieldLabel>
                <Select
                  value={taxCategory}
                  onValueChange={(val) => {
                    resetError("tax_category");
                    setTaxCategory(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("tax_category")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="standard">Általános</SelectItem>
                    <SelectItem value="reduced">Kedvezményes</SelectItem>
                    <SelectItem value="zero">Nulla kulcsos</SelectItem>
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"tax_category"} />
              </Field>

              <Field data-invalid={isInvalidField("legal_text")}>
                <FieldLabel htmlFor="legal_text">Jogi szöveg</FieldLabel>
                <Input
                  id="legal_text"
                  type="text"
                  placeholder="Pl.: Alanyi adómenetes"
                  value={legalText}
                  onChange={(e) => {
                    resetError("legal_text");
                    setLegalText(e.target.value);
                  }}
                  aria-invalid={isInvalidField("legal_text")}
                />
                <FieldError error={errors} field={"legal_text"} />
              </Field>

              <Field data-invalid={isInvalidField("reporting_code")}>
                <FieldLabel htmlFor="reporting_code">Jelentési kód</FieldLabel>
                <Input
                  id="reporting_code"
                  type="text"
                  value={reportingCode}
                  onChange={(e) => {
                    resetError("reporting_code");
                    setReportingCode(e.target.value);
                  }}
                  aria-invalid={isInvalidField("reporting_code")}
                />
                <FieldError error={errors} field={"reporting_code"} />
              </Field>

              <Field data-invalid={isInvalidField("status")}>
                <FieldLabel htmlFor="status">Státusz</FieldLabel>
                <Select
                  value={status}
                  onValueChange={(val) => {
                    resetError("status");
                    setStatus(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("status")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="active">Aktív</SelectItem>
                    <SelectItem value="inactive">Inaktív</SelectItem>
                    <SelectItem value="draft">Vázlat</SelectItem>
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"status"} />
              </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <div className="text-right mt-8 w-full">
              <Button className="mr-3" variant="outline" onClick={handleCancel}>
                Mégse
              </Button>
              <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
            </div>
          </Field>
        </form>
      </ConditionalCard>
    </>
  );
}
