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
} from "@/components/modules/tasks/lib/slice.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router-dom";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { useFormError } from "@/hooks/use_form_error.ts";
import type { SelectOptionList } from "@/lib/interfaces/common.ts";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { formatDateToYMD } from "@/lib/utils.ts";
import type { Task, TaskUserInput } from "./lib/interface";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import WorksheetsEdit from "@/components/modules/worksheets/Edit.tsx";
import ServicesEdit from "@/components/modules/services/Edit.tsx";
import TaxesEdit from "@/components/modules/taxes/Edit.tsx";
import type { Worksheet } from "../worksheets/lib/interface";
import type { Service } from "../services/lib/interface";
import type { Tax } from "../taxes/lib/interface";
import { Plus } from "lucide-react";
import { useNumberInput } from "@/hooks/use_number_input.ts";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (task: Task) => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
}: EditProps) {
  const [worksheetId, setWorksheetId] = React.useState("");
  const [serviceId, setServiceId] = React.useState("");
  const [currencyCode, setCurrencyCode] = React.useState("HUF");
  const [taxId, setTaxId] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const [priority, setPriority] = React.useState<string | null>("normal");
  const [dueDate, setDueDate] = React.useState<string | null>("");
  const [description, setDescription] = React.useState("");
  const [worksheetList, setWorksheetList] = React.useState<SelectOptionList>(
    [],
  );
  const [serviceList, setServiceList] = React.useState<SelectOptionList>([]);
  const [taxList, setTaxList] = React.useState<SelectOptionList>([]);
  const [currencyList, setCurrencyList] = React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { setListResponse } = useSelectList();
  const { errors, setErrors, unexpectedError } = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);
  const [openNewWorksheetDialog, setOpenNewWorksheetDialog] =
    React.useState(false);
  const [openNewServiceDialog, setOpenNewServiceDialog] = React.useState(false);
  const [openNewTaxDialog, setOpenNewTaxDialog] = React.useState(false);
  const quantity = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 2,
    allowEmpty: true,
  });
  const price = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 2,
    allowEmpty: true,
  });
  const handleEditWorksheetsSuccess = (worksheet: Worksheet) => {
    loadLists().then(() => {
      setTimeout(() => {
        setWorksheetId(worksheet.id);
      }, 0);
      setOpenNewWorksheetDialog(false);
    });
  };

  const handleEditServicesSuccess = (service: Service) => {
    loadLists().then(() => {
      setTimeout(() => {
        setServiceId(service.id);
      }, 0);
      setOpenNewServiceDialog(false);
    });
  };

  const handleEditTaxesSuccess = (tax: Tax) => {
    loadLists().then(() => {
      setTimeout(() => {
        setTaxId(tax.id);
      }, 0);
      setOpenNewTaxDialog(false);
    });
  };

  const prepareTaskInput = useCallback(
    (): TaskUserInput => ({
      id,
      worksheetId,
      serviceId,
      currencyCode,
      quantity: !isNaN(quantity.getNumericValue())
        ? quantity.getNumericValue().toString()
        : "",
      price: !isNaN(price.getNumericValue())
        ? price.getNumericValue().toString()
        : "",
      taxId,
      status,
      priority,
      dueDate,
      description,
    }),
    [
      id,
      worksheetId,
      serviceId,
      currencyCode,
      quantity,
      price,
      taxId,
      status,
      priority,
      dueDate,
      description,
    ],
  );

  const handleCreate = useCallback(() => {
    dispatch(create(prepareTaskInput())).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (
            typeof onSuccess === "function" &&
            typeof response.payload.jsonData?.data !== "undefined"
          ) {
            onSuccess(response.payload.jsonData.data);
          } else {
            navigate("/feladat/lista");
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
    dispatch,
    navigate,
    onSuccess,
    prepareTaskInput,
    setErrors,
    unexpectedError,
  ]);

  const handleUpdate = useCallback(() => {
    dispatch(update(prepareTaskInput())).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/feladat/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, navigate, prepareTaskInput, setErrors, unexpectedError]);

  const loadLists = useCallback(async () => {
    return Promise.all([
      dispatch(select_list("worksheets")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setWorksheetList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
      dispatch(select_list("services")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setServiceList, setErrors);
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
            setListResponse(response.payload, setTaxList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
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
    ]);
  }, [dispatch, setErrors, setListResponse, unexpectedError]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string") {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData?.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setWorksheetId(data.worksheet_id);
                setServiceId(data.service_id);
                setCurrencyCode(data.currency_code);
                setDescription(data.description ?? "");
                quantity.setValue(
                  data.quantity ? data.quantity.toString() : "",
                );
                price.setValue(data.price ? data.price.toString() : "");
                setTaxId(data.tax_id);
                setStatus(data.status);
                setPriority(data.priority);
                setDueDate(data.due_date ? formatDateToYMD(data.due_date) : "");
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
    // quantity and price are intentionally omitted to avoid infinite loops
    // They are only used to set initial values and don't need to trigger re-runs
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [dispatch, id, loadLists, setErrors, unexpectedError]);

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
      <Dialog
        open={openNewWorksheetDialog}
        onOpenChange={setOpenNewWorksheetDialog}
      >
        <DialogContent>
          <DialogTitle>Új munkalap létrehozása</DialogTitle>
          <WorksheetsEdit
            showCard={false}
            onSuccess={handleEditWorksheetsSuccess}
          />
        </DialogContent>
      </Dialog>
      <Dialog
        open={openNewServiceDialog}
        onOpenChange={setOpenNewServiceDialog}
      >
        <DialogContent>
          <DialogTitle>Új szolgáltatás létrehozása</DialogTitle>
          <ServicesEdit
            showCard={false}
            onSuccess={handleEditServicesSuccess}
          />
        </DialogContent>
      </Dialog>
      <Dialog open={openNewTaxDialog} onOpenChange={setOpenNewTaxDialog}>
        <DialogContent>
          <DialogTitle>Új adó létrehozása</DialogTitle>
          <TaxesEdit showCard={false} onSuccess={handleEditTaxesSuccess} />
        </DialogContent>
      </Dialog>
      <ConditionalCard
        showCard={showCard}
        title={`Feladat ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <Label htmlFor="worksheet_id">Munkalap</Label>
          <Select value={worksheetId} onValueChange={setWorksheetId}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {worksheetList.map((worksheet) => (
                <SelectItem key={worksheet.value} value={worksheet.value}>
                  {worksheet.title}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"worksheet_id"} />
          <Button
            type="button"
            variant="outline"
            onClick={() => setOpenNewWorksheetDialog(true)}
          >
            <Plus /> Új munkalap
          </Button>

          <Label htmlFor="service_id">Szolgáltatás</Label>
          <Select value={serviceId} onValueChange={setServiceId}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {serviceList.map((service) => (
                <SelectItem key={service.value} value={service.value}>
                  {service.title}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"service_id"} />
          <Button
            type="button"
            variant="outline"
            onClick={() => setOpenNewServiceDialog(true)}
          >
            <Plus /> Új szolgáltatás
          </Button>

          <Label htmlFor="description">Leírás</Label>
          <Input
            id="description"
            type="text"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
          <FieldError error={errors} field={"description"} />

          <Label htmlFor="currency_code">Pénznem</Label>
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

          <Label htmlFor="quantity">Munkaóra</Label>
          <Input
            id="quantity"
            type="text"
            value={quantity.displayValue}
            onChange={(e) =>
              quantity.handleInputChangeWithCursor(e.target.value, e.target)
            }
          />
          <FieldError error={errors} field={"quantity"} />

          <Label htmlFor="price">Egységár (nettó)</Label>
          <Input
            id="price"
            type="text"
            value={price.displayValue}
            onChange={(e) =>
              price.handleInputChangeWithCursor(e.target.value, e.target)
            }
          />
          <FieldError error={errors} field={"price"} />

          <Label htmlFor="tax_id">Adó</Label>
          <Select value={taxId} onValueChange={setTaxId}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {taxList.map((tax) => (
                <SelectItem key={tax.value} value={tax.value}>
                  {tax.title}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"tax_id"} />
          <Button
            type="button"
            variant="outline"
            onClick={() => setOpenNewTaxDialog(true)}
          >
            <Plus /> Új adó
          </Button>

          <Label htmlFor="status">Státusz</Label>
          <Select value={status} onValueChange={setStatus}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="active">Aktív</SelectItem>
              <SelectItem value="inactive">Inaktív</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"status"} />

          <Label htmlFor="priority">Prioritás</Label>
          <Select value={priority ?? ""} onValueChange={setPriority}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="low">Alacsony</SelectItem>
              <SelectItem value="normal">Normál</SelectItem>
              <SelectItem value="high">Magas</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"priority"} />

          <Label htmlFor="due_date">Határidő</Label>
          <Input
            id="due_date"
            type="date"
            value={dueDate ?? ""}
            onChange={(e) => setDueDate(e.target.value)}
          />
          <FieldError error={errors} field={"due_date"} />
          <div className="text-right mt-8">
            <Button
              className="mr-3"
              variant="outline"
              onClick={(e: React.FormEvent) => {
                e.preventDefault();
                navigate(-1);
              }}
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
