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
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    create,
    get,
    select_list,
    update,
} from "@/components/modules/inventory/lib/slice.ts";
import { type SelectOptionList } from "@/lib/interface.ts";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Inventory, InventoryError } from "./lib/interface";
import type { Product } from "../products/lib/interface";
import type { Warehouse } from "../warehouses/lib/interface";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import WarehousesEdit from "@/components/modules/warehouses/Edit.tsx";
import ProductsEdit from "@/components/modules/products/Edit.tsx";
import { Plus } from "lucide-react";
import { useNumberInput } from "@/hooks/use_number_input.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";

interface EditProps {
    showCard?: boolean;
    createOnly?: boolean;
    onSuccess?: (inventory: Inventory) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    createOnly = false,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [productId, setProductId] = React.useState("");
    const [warehouseId, setWarehouseId] = React.useState("");
    const [currencyCode, setCurrencyCode] = React.useState("HUF");
    const [status, setStatus] = React.useState("active");
    const [currencyList, setCurrencyList] = React.useState<SelectOptionList>(
        [],
    );
    const [productList, setProductList] = React.useState<SelectOptionList>([]);
    const [warehouseList, setWarehouseList] = React.useState<SelectOptionList>(
        [],
    );
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const [errors, setErrors] = React.useState<InventoryError | null>(null);
    const params = useParams();
    const id = createOnly ? null : (params["id"] ?? null);
    const [openNewProductDialog, setOpenNewProductDialog] =
        React.useState(false);
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

    const resetError = (field: keyof InventoryError) => {
        setErrors((previous) =>
            previous ? { ...previous, [field]: null } : previous,
        );
    };
    const isInvalidField = (field: keyof InventoryError) =>
        typeof errors?.[field] === "string";

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

    const handleCreate = async () => {
        const response = await dispatch(
            create({
                input: {
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
                },
            }),
        );
        if (create.fulfilled.match(response))
            handleApiResult(response.payload, {
                onSuccess: ({ data }) => {
                    toast.add({
                        type: "success",
                        title: "A mentés sikeresen megtörtént!",
                    });
                    if (onSuccess && data !== undefined) onSuccess(data);
                    else navigate("/raktarkeszlet/lista");
                },
                onFieldErrors: setErrors,
            });
    };

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

    const handleUpdate = async () => {
        const response = await dispatch(
            update({
                input: {
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
                },
            }),
        );
        if (update.fulfilled.match(response))
            handleApiResult(response.payload, {
                onSuccess: () => {
                    toast.add({
                        type: "success",
                        title: "A mentés sikeresen megtörtént!",
                    });
                    navigate("/raktarkeszlet/lista");
                },
                onFieldErrors: setErrors,
            });
    };

    const loadLists = useCallback(async () => {
        const lists = [
            ["currencies", setCurrencyList],
            ["products", setProductList],
            ["warehouses", setWarehouseList],
        ] as const;
        return Promise.all(
            lists.map(async ([listName, setList]) => {
                const response = await dispatch(
                    select_list({ list: listName }),
                );
                if (select_list.fulfilled.match(response)) {
                    handleApiResult(response.payload, {
                        onSuccess: ({ data }) => setList(data ?? []),
                    });
                }
            }),
        );
    }, [dispatch]);

    useEffect(() => {
        loadLists().then(() => {
            if (typeof id === "string") {
                dispatch(get({ uuid: id })).then((response) => {
                    if (get.fulfilled.match(response)) {
                        handleApiResult(response.payload, {
                            onSuccess: ({ data }) => {
                                if (!data) return;
                                setProductId(data.product_id);
                                setWarehouseId(data.warehouse_id);
                                minimumStockInput.setValue(
                                    data.minimum_stock
                                        ? data.minimum_stock.toString()
                                        : "",
                                );
                                maximumStockInput.setValue(
                                    data.maximum_stock
                                        ? data.maximum_stock.toString()
                                        : "",
                                );
                                setCurrencyCode(data.currency_code);
                                setStatus(data.status);
                            },
                        });
                    }
                });
            }
        });
        // minimumStockInput and maximumStockInput are intentionally omitted to avoid infinite loops
        // They are only used to set initial values and don't need to trigger re-runs
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch, id, loadLists]);

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
            <Dialog
                open={openNewProductDialog}
                onOpenChange={setOpenNewProductDialog}
            >
                <DialogContent>
                    <DialogTitle>Termék létrehozása</DialogTitle>
                    <ProductsEdit
                        showCard={false}
                        createOnly
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
                        createOnly
                        onSuccess={handleEditWarehousesSuccess}
                        onCancel={() => setOpenNewWarehouseDialog(false)}
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
                            {`Raktárkészlet ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            <Field data-invalid={isInvalidField("product_id")}>
                                <div className="flex items-center w-full">
                                    <div className="flex flex-1 items-center">
                                        <FieldLabel htmlFor="product_id">
                                            Termék
                                        </FieldLabel>
                                    </div>
                                    <div className="flex items-center">
                                        <Button
                                            type="button"
                                            variant="outline"
                                            onClick={() => {
                                                setOpenNewProductDialog(true);
                                            }}
                                        >
                                            <Plus />
                                        </Button>
                                    </div>
                                </div>
                                <Select
                                    value={productId}
                                    onValueChange={(val) => {
                                        resetError("product_id");
                                        setProductId(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "product_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {productList.map((product) => {
                                            return (
                                                <SelectItem
                                                    key={product.value}
                                                    value={product.value}
                                                >
                                                    {product.title}
                                                </SelectItem>
                                            );
                                        })}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.product_id} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("warehouse_id")}
                            >
                                <div className="flex items-center w-full">
                                    <div className="flex flex-1 items-center">
                                        <FieldLabel htmlFor="warehouse_id">
                                            Raktár
                                        </FieldLabel>
                                    </div>
                                    <div className="flex items-center">
                                        <Button
                                            type="button"
                                            variant="outline"
                                            onClick={() =>
                                                setOpenNewWarehouseDialog(true)
                                            }
                                        >
                                            <Plus />
                                        </Button>
                                    </div>
                                </div>
                                <Select
                                    value={warehouseId}
                                    onValueChange={(val) => {
                                        resetError("warehouse_id");
                                        setWarehouseId(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "warehouse_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {warehouseList.map((warehouse) => {
                                            return (
                                                <SelectItem
                                                    key={warehouse.value}
                                                    value={warehouse.value}
                                                >
                                                    {warehouse.title}
                                                </SelectItem>
                                            );
                                        })}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.warehouse_id} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("minimum_stock")}
                            >
                                <FieldLabel htmlFor="minimum_stock">
                                    Minimum készlet
                                </FieldLabel>
                                <Input
                                    id="minimum_stock"
                                    type="text"
                                    placeholder="10"
                                    value={minimumStockInput.displayValue}
                                    onChange={(e) => {
                                        resetError("minimum_stock");
                                        minimumStockInput.handleInputChangeWithCursor(
                                            e.target.value,
                                            e.target,
                                        );
                                    }}
                                    aria-invalid={isInvalidField(
                                        "minimum_stock",
                                    )}
                                />
                                <FieldErrorV2 error={errors?.minimum_stock} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("maximum_stock")}
                            >
                                <FieldLabel htmlFor="maximum_stock">
                                    Maximum készlet
                                </FieldLabel>
                                <Input
                                    id="maximum_stock"
                                    type="text"
                                    placeholder="100"
                                    value={maximumStockInput.displayValue}
                                    onChange={(e) => {
                                        resetError("maximum_stock");
                                        maximumStockInput.handleInputChangeWithCursor(
                                            e.target.value,
                                            e.target,
                                        );
                                    }}
                                    aria-invalid={isInvalidField(
                                        "maximum_stock",
                                    )}
                                />
                                <FieldErrorV2 error={errors?.maximum_stock} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("currency_code")}
                            >
                                <FieldLabel htmlFor="currency_code">
                                    Pénznem
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
                                        aria-invalid={isInvalidField(
                                            "currency_code",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {currencyList.map((currency) => {
                                            return (
                                                <SelectItem
                                                    key={currency.value}
                                                    value={currency.value}
                                                >
                                                    {currency.title}
                                                </SelectItem>
                                            );
                                        })}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.currency_code} />
                            </Field>

                            <Field data-invalid={isInvalidField("status")}>
                                <FieldLabel htmlFor="status">
                                    Státusz
                                </FieldLabel>
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
                                        <SelectItem value="active">
                                            Aktív
                                        </SelectItem>
                                        <SelectItem value="discontinued">
                                            Kivezetett
                                        </SelectItem>
                                        <SelectItem value="inactive">
                                            Inaktív
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.status} />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <Field orientation="horizontal">
                        <div className="text-right mt-8 w-full">
                            <Button
                                className="mr-3"
                                variant="outline"
                                onClick={handleCancel}
                            >
                                Mégse
                            </Button>
                            <Button type="submit">
                                {id ? "Módosítás" : "Létrehozás"}
                            </Button>
                        </div>
                    </Field>
                </form>
            </ConditionalCard>
        </>
    );
}
