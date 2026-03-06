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
} from "@/components/modules/inventory/lib/slice.ts";
import { type SelectOptionList } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useNavigate } from "react-router-dom";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Inventory } from "./lib/interface";
import type { Product } from "../products/lib/interface";
import type { Warehouse } from "../warehouses/lib/interface";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import WarehousesEdit from "@/components/modules/warehouses/Edit.tsx";
import ProductsEdit from "@/components/modules/products/Edit.tsx";
import { Plus } from "lucide-react";
import { useNumberInput } from "@/hooks/use_number_input.ts";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (inventory: Inventory) => void;
  onCancel?: () => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
  onCancel = undefined,
}: EditProps) {
  const [productId, setProductId] = React.useState("");
  const [warehouseId, setWarehouseId] = React.useState("");
  const [currencyCode, setCurrencyCode] = React.useState("");
  const [status, setStatus] = React.useState("");
  const [currencyList, setCurrencyList] = React.useState<SelectOptionList>([]);
  const [productList, setProductList] = React.useState<SelectOptionList>([]);
  const [warehouseList, setWarehouseList] = React.useState<SelectOptionList>(
    [],
  );
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { setListResponse } = useSelectList();
  const { errors, setErrors, unexpectedError } = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);
  const [openNewProductDialog, setOpenNewProductDialog] = React.useState(false);
  const [openNewWarehouseDialog, setOpenNewWarehouseDialog] =
    React.useState(false);

  const minimumStockInput = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 0,
    allowEmpty: true,
  });

  const maximumStockInput = useNumberInput({
    showThousandSeparator: true,
    decimalPlaces: 0,
    allowEmpty: true,
  });

  const handleEditProductsSuccess = async (product: Product) => {
    return loadLists().then(() => {
      setTimeout(() => {
        setProductId(product.id);
      }, 0);
      setOpenNewProductDialog(false);
    });
  };

  const handleEditWarehousesSuccess = async (warehouse: Warehouse) => {
    return loadLists().then(() => {
      setTimeout(() => {
        setWarehouseId(warehouse.id);
      }, 0);
      setOpenNewWarehouseDialog(false);
    });
  };

  const handleCreate = useCallback(() => {
    dispatch(
      create({
        id,
        productId,
        warehouseId,
        minimumStock: !isNaN(minimumStockInput.getNumericValue())
          ? minimumStockInput.getNumericValue().toString()
          : "",
        maximumStock: !isNaN(maximumStockInput.getNumericValue())
          ? maximumStockInput.getNumericValue().toString()
          : "",
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
            navigate("/raktarkeszlet/lista");
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
    minimumStockInput,
    dispatch,
    id,
    productId,
    warehouseId,
    maximumStockInput,
    currencyCode,
    status,
    onSuccess,
    navigate,
    setErrors,
    unexpectedError,
  ]);

  const handleCancel = useCallback(
    (e: React.FormEvent) => {
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
        productId,
        warehouseId,
        minimumStock: !isNaN(minimumStockInput.getNumericValue())
          ? minimumStockInput.getNumericValue().toString()
          : "",
        maximumStock: !isNaN(maximumStockInput.getNumericValue())
          ? maximumStockInput.getNumericValue().toString()
          : "",
        currencyCode,
        status,
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/raktarkeszlet/lista");
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
    id,
    productId,
    warehouseId,
    minimumStockInput,
    maximumStockInput,
    currencyCode,
    status,
    navigate,
    setErrors,
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
      dispatch(select_list("products")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setProductList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
      dispatch(select_list("warehouses")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setWarehouseList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
    ]);
  }, [dispatch, setListResponse, setErrors, unexpectedError]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string") {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData?.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setProductId(data.product_id);
                setWarehouseId(data.warehouse_id);
                minimumStockInput.setValue(
                  data.minimum_stock ? data.minimum_stock.toString() : "",
                );
                maximumStockInput.setValue(
                  data.maximum_stock ? data.maximum_stock.toString() : "",
                );
                setCurrencyCode(data.currency_code);
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
    // minimumStockInput and maximumStockInput are intentionally omitted to avoid infinite loops
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
      <Dialog
        open={openNewProductDialog}
        onOpenChange={setOpenNewProductDialog}
      >
        <DialogContent>
          <DialogTitle>Termék létrehozása</DialogTitle>
          <ProductsEdit
            showCard={false}
            onSuccess={handleEditProductsSuccess}
            onCancel={() => setOpenNewProductDialog(false)}
          />
        </DialogContent>
      </Dialog>
      <Dialog
        open={openNewWarehouseDialog}
        onOpenChange={setOpenNewWarehouseDialog}
      >
        <DialogContent>
          <DialogTitle>Raktár létrehozása</DialogTitle>
          <WarehousesEdit
            showCard={false}
            onSuccess={handleEditWarehousesSuccess}
            onCancel={() => setOpenNewWarehouseDialog(false)}
          />
        </DialogContent>
      </Dialog>
      <ConditionalCard
        showCard={showCard}
        title={`Raktárkészlet ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <div className="flex items-center w-full">
            <div className="flex flex-1 items-center">
              <Label htmlFor="product_id">Termék</Label>
            </div>
            <div className="flex items-center">
              <Button
                type="button"
                variant="outline"
                onClick={() => setOpenNewProductDialog(true)}
              >
                <Plus />
              </Button>
            </div>
          </div>
          <Select value={productId} onValueChange={(val) => setProductId(val)}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {productList.map((product) => {
                return (
                  <SelectItem key={product.value} value={product.value}>
                    {product.title}
                  </SelectItem>
                );
              })}
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"product_id"} />
          <div className="flex items-center w-full">
            <div className="flex flex-1 items-center">
              <Label htmlFor="warehouse_id">Raktár</Label>
            </div>
            <div className="flex items-center">
              <Button
                type="button"
                variant="outline"
                onClick={() => setOpenNewWarehouseDialog(true)}
              >
                <Plus />
              </Button>
            </div>
          </div>
          <Select
            value={warehouseId}
            onValueChange={(val) => setWarehouseId(val)}
          >
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {warehouseList.map((warehouse) => {
                return (
                  <SelectItem key={warehouse.value} value={warehouse.value}>
                    {warehouse.title}
                  </SelectItem>
                );
              })}
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"warehouse_id"} />
          <Label htmlFor="minimum_stock">Minimum készlet</Label>
          <Input
            id="minimum_stock"
            type="text"
            placeholder="10"
            value={minimumStockInput.displayValue}
            onChange={(e) =>
              minimumStockInput.handleInputChangeWithCursor(
                e.target.value,
                e.target,
              )
            }
          />
          <FieldError error={errors} field={"minimum_stock"} />
          <Label htmlFor="maximum_stock">Maximum készlet</Label>
          <Input
            id="maximum_stock"
            type="text"
            placeholder="100"
            value={maximumStockInput.displayValue}
            onChange={(e) =>
              maximumStockInput.handleInputChangeWithCursor(
                e.target.value,
                e.target,
              )
            }
          />
          <FieldError error={errors} field={"maximum_stock"} />
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

          <Label htmlFor="status">Állapot</Label>
          <Select value={status} onValueChange={(val) => setStatus(val)}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="active">Aktív</SelectItem>
              <SelectItem value="inactive">Inaktív</SelectItem>
              <SelectItem value="discontinued">Kivezetett</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"status"} />
          <div className="text-right mt-8">
            <Button className="mr-3" variant="outline" onClick={handleCancel}>
              Mégse
            </Button>
            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </div>
        </form>
      </ConditionalCard>
    </>
  );
}
