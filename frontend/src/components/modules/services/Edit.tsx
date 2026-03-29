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
} from "@/components/modules/services/lib/slice.ts";
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
import type { SelectOptionList } from "@/lib/interfaces/common.ts";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Service } from "./lib/interface";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import TaxesEdit from "@/components/modules/taxes/Edit.tsx";
import { Plus } from "lucide-react";
import type { Tax } from "../taxes/lib/interface";
import { useNumberInput } from "@/hooks/use_number_input.ts";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (service: Service) => void;
  onCancel?: () => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
  onCancel = undefined,
}: EditProps) {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [defaultTaxId, setDefaultTaxId] = React.useState("");
  const [currencyCode, setCurrencyCode] = React.useState("");
  const [status, setStatus] = React.useState("");
  const [currencyList, setCurrencyList] = React.useState<SelectOptionList>([]);
  const [taxesList, setTaxesList] = React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { setListResponse } = useSelectList();
  const { errors, setErrors, unexpectedError, isInvalidField, resetError } =
    useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);
  const [openNewTaxDialog, setOpenNewTaxDialog] = React.useState(false);
  const defaultPrice = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 2,
    allowEmpty: true,
  });
  const handleEditTaxesSuccess = async (tax: Tax) => {
    return loadLists().then(() => {
      setTimeout(() => {
        setDefaultTaxId(tax.id);
      }, 0);
      setOpenNewTaxDialog(false);
    });
  };

  const handleCreate = useCallback(() => {
    dispatch(
      create({
        id,
        name,
        description,
        defaultPrice: !isNaN(defaultPrice.getNumericValue())
          ? defaultPrice.getNumericValue().toString()
          : "",
        defaultTaxId,
        currencyCode,
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
            navigate("/szolgaltatas/lista");
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
    currencyCode,
    defaultPrice,
    defaultTaxId,
    description,
    dispatch,
    id,
    name,
    navigate,
    onSuccess,
    setErrors,
    status,
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
        name,
        description,
        defaultPrice: !isNaN(defaultPrice.getNumericValue())
          ? defaultPrice.getNumericValue().toString()
          : "",
        defaultTaxId,
        currencyCode,
        status,
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/szolgaltatas/lista");
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
    currencyCode,
    defaultPrice,
    defaultTaxId,
    description,
    dispatch,
    id,
    name,
    navigate,
    setErrors,
    status,
    unexpectedError,
  ]);

  const loadLists = useCallback(async () => {
    return Promise.all([
      dispatch(select_list("currencies")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setCurrencyList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
      dispatch(select_list("taxes")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setTaxesList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
    ]);
  }, [dispatch, setErrors, unexpectedError, setListResponse]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string") {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData?.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setName(data.name);
                setDescription(data.description ?? "");
                defaultPrice.setValue(data.default_price ?? "");
                setDefaultTaxId(data.default_tax_id ?? "");
                setCurrencyCode(data.currency_code ?? "");
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
    // defaultPrice is intentionally omitted to avoid infinite loops
    // They are only used to set initial values and don't need to trigger re-runs
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [dispatch, id, setErrors, unexpectedError, loadLists]);

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
      <Dialog open={openNewTaxDialog} onOpenChange={setOpenNewTaxDialog}>
        <DialogContent>
          <DialogTitle>Adó létrehozása</DialogTitle>
          <TaxesEdit
            showCard={false}
            onSuccess={handleEditTaxesSuccess}
            onCancel={() => setOpenNewTaxDialog(false)}
          />
        </DialogContent>
      </Dialog>
      <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <FieldSet>
            <FieldLegend>
              {`Szolgáltatás ${id ? "módosítás" : "létrehozás"}`}
            </FieldLegend>
            <FieldGroup>
              <Field data-invalid={isInvalidField("name")}>
                <FieldLabel htmlFor="name">Megnevezés</FieldLabel>
                <Input
                  id="name"
                  type="text"
                  value={name}
                  onChange={(e) => {
                    resetError("name");
                    setName(e.target.value);
                  }}
                  aria-invalid={isInvalidField("name")}
                />
                <FieldError error={errors} field={"name"} />
              </Field>

              <Field data-invalid={isInvalidField("description")}>
                <FieldLabel htmlFor="description">Leírás</FieldLabel>
                <Input
                  id="description"
                  type="text"
                  value={description}
                  onChange={(e) => {
                    resetError("description");
                    setDescription(e.target.value);
                  }}
                  aria-invalid={isInvalidField("description")}
                />
                <FieldError error={errors} field={"description"} />
              </Field>

              <Field data-invalid={isInvalidField("default_price")}>
                <FieldLabel htmlFor="default_price">
                  Alapértelmezett ár
                </FieldLabel>
                <Input
                  id="default_price"
                  type="text"
                  placeholder="1 000"
                  value={defaultPrice.displayValue}
                  onChange={(e) => {
                    resetError("default_price");
                    defaultPrice.handleInputChangeWithCursor(
                      e.target.value,
                      e.target,
                    );
                  }}
                  aria-invalid={isInvalidField("default_price")}
                />
                <FieldError error={errors} field={"default_price"} />
              </Field>
              <Field data-invalid={isInvalidField("default_tax_id")}>
                <div className="flex items-center w-full">
                  <div className="flex flex-1 items-center">
                    <FieldLabel htmlFor="default_tax_id">
                      Alapértelmezett adózás
                    </FieldLabel>
                  </div>
                  <div className="flex items-center">
                    <Button
                      type="button"
                      variant="outline"
                      onClick={() => setOpenNewTaxDialog(true)}
                    >
                      <Plus />
                    </Button>
                  </div>
                </div>
                <Select
                  value={defaultTaxId}
                  onValueChange={(val) => {
                    resetError("default_tax_id");
                    setDefaultTaxId(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("default_tax_id")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {taxesList.map((tax) => {
                      return (
                        <SelectItem key={tax.value} value={tax.value}>
                          {tax.title}
                        </SelectItem>
                      );
                    })}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"default_tax_id"} />
              </Field>

              <Field data-invalid={isInvalidField("currency_code")}>
                <FieldLabel htmlFor="currency_code">
                  Alapértelmezett pénznem
                </FieldLabel>
                <Select
                  value={currencyCode}
                  onValueChange={(val) => {
                    resetError("currency_code");
                    setCurrencyCode(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("currency_code")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {currencyList.map((currency) => {
                      return (
                        <SelectItem key={currency.value} value={currency.value}>
                          {currency.title}
                        </SelectItem>
                      );
                    })}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"currency_code"} />
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
                    <SelectItem value="lead">Érdeklődő</SelectItem>
                    <SelectItem value="prospect">Lehetséges vevő</SelectItem>
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
