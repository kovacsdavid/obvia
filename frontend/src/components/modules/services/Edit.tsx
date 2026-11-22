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
} from "@/components/modules/services/lib/slice.ts";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router";
import { useParams } from "react-router";
import type { SelectOptionList } from "@/lib/interface.ts";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Service, ServiceError } from "./lib/interface";
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
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";

interface EditProps {
    showCard?: boolean;
    createOnly?: boolean;
    onSuccess?: (service: Service) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    createOnly = false,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [name, setName] = React.useState("");
    const [description, setDescription] = React.useState("");
    const [defaultTaxId, setDefaultTaxId] = React.useState("");
    const [currencyCode, setCurrencyCode] = React.useState("HUF");
    const [status, setStatus] = React.useState("active");
    const [unitOfMeasureId, setUnitOfMeasureId] = React.useState("");
    const [newUnitOfMeasure, setNewUnitOfMeasure] = React.useState("");
    const [unitsOfMeasureList, setUnitsOfMeasureList] =
        React.useState<SelectOptionList>([]);
    const [currencyList, setCurrencyList] = React.useState<SelectOptionList>(
        [],
    );
    const [taxesList, setTaxesList] = React.useState<SelectOptionList>([]);
    const [errors, setErrors] = React.useState<ServiceError | null>(null);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const params = useParams();
    const id = createOnly ? null : (params["id"] ?? null);
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

    const handleCreate = async () => {
        const result = await dispatch(
            create({
                input: {
                    id,
                    name,
                    description,
                    defaultPrice: !isNaN(defaultPrice.getNumericValue())
                        ? defaultPrice.getNumericValue().toString()
                        : "",
                    defaultTaxId,
                    currencyCode,
                    status,
                    unitOfMeasureId,
                    newUnitOfMeasure,
                },
            }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: ({ data }) => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                if (
                    typeof onSuccess === "function" &&
                    typeof data !== "undefined"
                ) {
                    onSuccess(data);
                } else {
                    navigate("/szolgaltatas/lista");
                }
            },
            onFieldErrors: (fields: ServiceError) => setErrors(fields),
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
        const result = await dispatch(
            update({
                input: {
                    id,
                    name,
                    description,
                    defaultPrice: !isNaN(defaultPrice.getNumericValue())
                        ? defaultPrice.getNumericValue().toString()
                        : "",
                    defaultTaxId,
                    currencyCode,
                    status,
                    unitOfMeasureId,
                    newUnitOfMeasure,
                },
            }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/szolgaltatas/lista");
            },
            onFieldErrors: (fields: ServiceError) => setErrors(fields),
        });
    };

    const loadLists = useCallback(async () => {
        return Promise.all([
            dispatch(select_list({ list: "currencies" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => setCurrencyList(data ?? []),
                    });
                }),
            dispatch(select_list({ list: "taxes" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => setTaxesList(data ?? []),
                    });
                }),
            dispatch(select_list({ list: "units_of_measure" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) =>
                            setUnitsOfMeasureList(data ?? []),
                    });
                }),
        ]);
    }, [dispatch]);

    useEffect(() => {
        loadLists().then(async () => {
            if (typeof id === "string") {
                const result = await dispatch(get({ uuid: id })).unwrap();
                handleApiResult(result, {
                    onSuccess: ({ data }) => {
                        if (typeof data !== "undefined") {
                            setName(data.name);
                            setDescription(data.description ?? "");
                            defaultPrice.setValue(data.default_price ?? "");
                            setDefaultTaxId(data.default_tax_id ?? "");
                            setCurrencyCode(data.currency_code ?? "");
                            setStatus(data.status);
                            setUnitOfMeasureId(data.unit_of_measure_id);
                        }
                    },
                });
            }
        });
        // defaultPrice is intentionally omitted to avoid infinite loops
        // They are only used to set initial values and don't need to trigger re-runs
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch, id, loadLists]);

    const resetError = (field: keyof ServiceError) => {
        setErrors((previous) =>
            previous && previous[field] != null
                ? { ...previous, [field]: null }
                : previous,
        );
    };

    const isInvalidField = (field: keyof ServiceError) =>
        typeof errors?.[field] === "string";

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
            <Dialog open={openNewTaxDialog} onOpenChange={setOpenNewTaxDialog}>
                <DialogContent>
                    <DialogTitle>Adó létrehozása</DialogTitle>
                    <TaxesEdit
                        showCard={false}
                        createOnly
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
                                <FieldLabel htmlFor="name">
                                    Megnevezés
                                </FieldLabel>
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
                                <FieldErrorV2 error={errors?.name} />
                            </Field>

                            <Field data-invalid={isInvalidField("description")}>
                                <FieldLabel htmlFor="description">
                                    Leírás
                                </FieldLabel>
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
                                <FieldErrorV2 error={errors?.description} />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("default_price")}
                            >
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
                                    aria-invalid={isInvalidField(
                                        "default_price",
                                    )}
                                />
                                <FieldErrorV2 error={errors?.default_price} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("default_tax_id")}
                            >
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
                                            onClick={() =>
                                                setOpenNewTaxDialog(true)
                                            }
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
                                        aria-invalid={isInvalidField(
                                            "default_tax_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {taxesList.map((tax) => {
                                            return (
                                                <SelectItem
                                                    key={tax.value}
                                                    value={tax.value}
                                                >
                                                    {tax.title}
                                                </SelectItem>
                                            );
                                        })}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.default_tax_id} />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("currency_code")}
                            >
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

                            <Field
                                data-invalid={isInvalidField(
                                    "unit_of_measure_id",
                                )}
                            >
                                <FieldLabel htmlFor="unit_of_measure_id">
                                    Mértékegység
                                </FieldLabel>
                                <Select
                                    value={unitOfMeasureId}
                                    onValueChange={(val) => {
                                        resetError("unit_of_measure_id");
                                        setUnitOfMeasureId(val);
                                    }}
                                >
                                    <SelectTrigger
                                        id="unit_of_measure_id"
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "unit_of_measure_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {unitsOfMeasureList.map(
                                            (unit_of_measure) => {
                                                return (
                                                    <SelectItem
                                                        key={
                                                            unit_of_measure.value
                                                        }
                                                        value={
                                                            unit_of_measure.value
                                                        }
                                                    >
                                                        {unit_of_measure.title}
                                                    </SelectItem>
                                                );
                                            },
                                        )}
                                        <SelectItem value="other">
                                            Egyéb
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2
                                    error={errors?.unit_of_measure_id}
                                />
                            </Field>

                            {unitOfMeasureId === "other" ? (
                                <>
                                    <Field
                                        data-invalid={isInvalidField(
                                            "new_unit_of_measure",
                                        )}
                                    >
                                        <FieldLabel htmlFor="new_unit_of_measure">
                                            Egyéb mértékegység
                                        </FieldLabel>
                                        <Input
                                            id="new_unit_of_measure"
                                            type="text"
                                            value={newUnitOfMeasure}
                                            onChange={(e) => {
                                                resetError(
                                                    "new_unit_of_measure",
                                                );
                                                setNewUnitOfMeasure(
                                                    e.target.value,
                                                );
                                            }}
                                            aria-invalid={isInvalidField(
                                                "new_unit_of_measure",
                                            )}
                                        />
                                        <FieldErrorV2
                                            error={errors?.new_unit_of_measure}
                                        />
                                    </Field>
                                </>
                            ) : null}

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
