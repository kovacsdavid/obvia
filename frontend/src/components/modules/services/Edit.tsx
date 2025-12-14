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
import { Button, FieldError, GlobalError, Input, Label } from "@/components/ui";
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

interface EditProps {
  showCard?: boolean;
  onSuccess?: (service: Service) => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
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
  const { errors, setErrors, unexpectedError } = useFormError();
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
      <GlobalError error={errors} />
      <Dialog open={openNewTaxDialog} onOpenChange={setOpenNewTaxDialog}>
        <DialogContent>
          <DialogTitle>Új adó létrehozása</DialogTitle>
          <TaxesEdit showCard={false} onSuccess={handleEditTaxesSuccess} />
        </DialogContent>
      </Dialog>
      <ConditionalCard
        showCard={showCard}
        title={`Szolgáltatás ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <Label htmlFor="name">Megnevezés</Label>
          <Input
            id="name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <FieldError error={errors} field={"name"} />

          <Label htmlFor="description">Leírás</Label>
          <Input
            id="description"
            type="text"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
          <FieldError error={errors} field={"description"} />

          <Label htmlFor="default_price">Alapértelmezett ár</Label>
          <Input
            id="default_price"
            type="text"
            value={defaultPrice.displayValue}
            onChange={(e) =>
              defaultPrice.handleInputChangeWithCursor(e.target.value, e.target)
            }
          />
          <FieldError error={errors} field={"default_price"} />

          <Label htmlFor="default_tax_id">Alapértelmezett adózás</Label>
          <Select
            value={defaultTaxId}
            onValueChange={(val) => setDefaultTaxId(val)}
          >
            <SelectTrigger className={"w-full"}>
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
          <Button
            type="button"
            variant="outline"
            onClick={() => setOpenNewTaxDialog(true)}
          >
            <Plus /> Új adó
          </Button>

          <Label htmlFor="currency_code">Alapértelmezett pénznem</Label>
          <Select
            value={currencyCode}
            onValueChange={(val) => setCurrencyCode(val)}
          >
            <SelectTrigger className={"w-full"}>
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

          <Label htmlFor="status">Státusz</Label>
          <Select value={status} onValueChange={(val) => setStatus(val)}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="active">Aktív</SelectItem>
              <SelectItem value="lead">Érdeklődő</SelectItem>
              <SelectItem value="prospect">Lehetséges vevő</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"status"} />
          <div className="text-right mt-8">
            <Button
              className="mr-3"
              type="submit"
              variant="outline"
              onClick={() => navigate(-1)}
            >
              Mégse
            </Button>
            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </div>
        </form>
      </ConditionalCard>
    </>
  );
}
