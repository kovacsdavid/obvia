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
} from "@/components/modules/tasks/lib/slice.ts";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router";
import { useSelectList } from "@/hooks/use_select_list.ts";
import type { SelectOptionList } from "@/lib/interface.ts";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { formatDateToYMD } from "@/lib/utils.ts";
import type { Task, TaskErrors, TaskUserInput } from "./lib/interface";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import WorksheetsEdit from "@/components/modules/worksheets/Edit.tsx";
import ServicesEdit from "@/components/modules/services/Edit.tsx";
import TaxesEdit from "@/components/modules/taxes/Edit.tsx";
import type { Worksheet } from "@/components/modules/worksheets/lib/interface";
import type { Service } from "@/components/modules/services/lib/interface";
import type { Tax } from "@/components/modules/taxes/lib/interface";
import { Plus } from "lucide-react";
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
    onSuccess?: (task: Task) => void;
}

export default function Edit({
    showCard = true,
    onSuccess = undefined,
}: EditProps) {
    const params = useParams();
    const referenceId = React.useMemo(() => params["referenceId"], [params]);
    const [worksheetId, setWorksheetId] = React.useState(referenceId ?? "");
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
    const [currencyList, setCurrencyList] = React.useState<SelectOptionList>(
        [],
    );
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const { setListResponse } = useSelectList();
    const [errors, setErrors] = React.useState<TaskErrors | null>(null);
    const id = React.useMemo(() => params["id"] ?? null, [params]);
    const [openNewWorksheetDialog, setOpenNewWorksheetDialog] =
        React.useState(false);
    const [openNewServiceDialog, setOpenNewServiceDialog] =
        React.useState(false);
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

    const prepareTaskInput = (): TaskUserInput => ({
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
    });

    const handleCreate = async () => {
        const result = await dispatch(
            create({ input: prepareTaskInput() }),
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
                    navigate("/feladat/lista");
                }
            },
            onFieldErrors: setErrors,
        });
    };

    const handleUpdate = async () => {
        const result = await dispatch(
            update({ input: prepareTaskInput() }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/feladat/lista");
            },
        });
    };

    const loadLists = useCallback(async () => {
        return Promise.all([
            dispatch(select_list({ list: "worksheets" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setWorksheetList(data);
                            } else {
                                setWorksheetList([]);
                            }
                        },
                    });
                }),
            dispatch(select_list({ list: "services" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setServiceList(data);
                            } else {
                                setServiceList([]);
                            }
                        },
                    });
                }),
            dispatch(select_list({ list: "taxes" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setTaxList(data);
                            } else {
                                setTaxList([]);
                            }
                        },
                    });
                }),
            dispatch(select_list({ list: "currencies" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setCurrencyList(data);
                            } else {
                                setCurrencyList([]);
                            }
                        },
                    });
                }),
        ]);
    }, [dispatch, setErrors, setListResponse]);

    useEffect(() => {
        loadLists().then(async () => {
            if (typeof id === "string") {
                const result = await dispatch(get({ uuid: id })).unwrap();

                handleApiResult(result, {
                    onSuccess: ({ data }) => {
                        if (typeof data !== "undefined") {
                            setWorksheetId(data.worksheet_id);
                            setServiceId(data.service_id);
                            setCurrencyCode(data.currency_code);
                            setDescription(data.description ?? "");
                            quantity.setValue(
                                data.quantity ? data.quantity.toString() : "",
                            );
                            price.setValue(
                                data.price ? data.price.toString() : "",
                            );
                            setTaxId(data.tax_id);
                            setStatus(data.status);
                            setPriority(data.priority);
                            setDueDate(
                                data.due_date
                                    ? formatDateToYMD(data.due_date)
                                    : "",
                            );
                        }
                    },
                });
            }
        });
        // quantity and price are intentionally omitted to avoid infinite loops
        // They are only used to set initial values and don't need to trigger re-runs
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch, id, loadLists, setErrors]);

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        if (typeof id === "string") {
            handleUpdate();
        } else {
            handleCreate();
        }
    };

    const clearError = (field: keyof TaskErrors) => {
        setErrors((prev) =>
            prev && prev[field] != null ? { ...prev, [field]: null } : prev,
        );
    };

    const isInvalidField = (field: keyof TaskErrors) =>
        typeof errors?.[field] === "string";

    return (
        <>
            <Dialog
                open={openNewWorksheetDialog}
                onOpenChange={setOpenNewWorksheetDialog}
            >
                <DialogContent>
                    <DialogTitle>Munkalap létrehozása</DialogTitle>
                    <WorksheetsEdit
                        showCard={false}
                        onSuccess={handleEditWorksheetsSuccess}
                        onCancel={() => setOpenNewWorksheetDialog(false)}
                    />
                </DialogContent>
            </Dialog>
            <Dialog
                open={openNewServiceDialog}
                onOpenChange={setOpenNewServiceDialog}
            >
                <DialogContent>
                    <DialogTitle>Szolgáltatás létrehozása</DialogTitle>
                    <ServicesEdit
                        showCard={false}
                        onSuccess={handleEditServicesSuccess}
                        onCancel={() => setOpenNewServiceDialog(false)}
                    />
                </DialogContent>
            </Dialog>
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
                            {`Feladat ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            {typeof referenceId !== "string" ? (
                                <Field
                                    data-invalid={isInvalidField(
                                        "worksheet_id",
                                    )}
                                >
                                    <div className="flex items-center w-full">
                                        <div className="flex flex-1 items-center">
                                            <FieldLabel htmlFor="worksheet_id">
                                                Munkalap
                                            </FieldLabel>
                                        </div>
                                        <div className="flex items-center">
                                            <Button
                                                type="button"
                                                variant="outline"
                                                onClick={() =>
                                                    setOpenNewWorksheetDialog(
                                                        true,
                                                    )
                                                }
                                            >
                                                <Plus />
                                            </Button>
                                        </div>
                                    </div>
                                    <Select
                                        value={worksheetId}
                                        onValueChange={(val) => {
                                            clearError("worksheet_id");
                                            setWorksheetId(val);
                                        }}
                                    >
                                        <SelectTrigger
                                            className={"w-full"}
                                            aria-invalid={isInvalidField(
                                                "worksheet_id",
                                            )}
                                        >
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            {worksheetList.map((worksheet) => (
                                                <SelectItem
                                                    key={worksheet.value}
                                                    value={worksheet.value}
                                                >
                                                    {worksheet.title}
                                                </SelectItem>
                                            ))}
                                        </SelectContent>
                                    </Select>
                                    <FieldErrorV2
                                        error={errors?.worksheet_id}
                                    />
                                </Field>
                            ) : null}
                            <Field data-invalid={isInvalidField("service_id")}>
                                <div className="flex items-center w-full">
                                    <div className="flex flex-1 items-center">
                                        <FieldLabel htmlFor="service_id">
                                            Szolgáltatás
                                        </FieldLabel>
                                    </div>
                                    <div className="flex items-center">
                                        <Button
                                            type="button"
                                            variant="outline"
                                            onClick={() =>
                                                setOpenNewServiceDialog(true)
                                            }
                                        >
                                            <Plus />
                                        </Button>
                                    </div>
                                </div>
                                <Select
                                    value={serviceId}
                                    onValueChange={(val) => {
                                        clearError("service_id");
                                        setServiceId(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "service_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {serviceList.map((service) => (
                                            <SelectItem
                                                key={service.value}
                                                value={service.value}
                                            >
                                                {service.title}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.service_id} />
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
                                        clearError("description");
                                        setDescription(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField("description")}
                                />
                                <FieldErrorV2 error={errors?.description} />
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
                                        clearError("currency_code");
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
                            <Field data-invalid={isInvalidField("quantity")}>
                                <FieldLabel htmlFor="quantity">
                                    Munkaóra
                                </FieldLabel>
                                <Input
                                    id="quantity"
                                    type="text"
                                    placeholder="10"
                                    value={quantity.displayValue}
                                    onChange={(e) => {
                                        clearError("quantity");
                                        quantity.handleInputChangeWithCursor(
                                            e.target.value,
                                            e.target,
                                        );
                                    }}
                                    aria-invalid={isInvalidField("quantity")}
                                />
                                <FieldErrorV2 error={errors?.quantity} />
                            </Field>
                            <Field data-invalid={isInvalidField("price")}>
                                <FieldLabel htmlFor="price">
                                    Egységár (nettó)
                                </FieldLabel>
                                <Input
                                    id="price"
                                    type="text"
                                    placeholder="1 000"
                                    value={price.displayValue}
                                    onChange={(e) => {
                                        clearError("price");
                                        price.handleInputChangeWithCursor(
                                            e.target.value,
                                            e.target,
                                        );
                                    }}
                                    aria-invalid={isInvalidField("price")}
                                />
                                <FieldErrorV2 error={errors?.price} />
                            </Field>
                            <Field data-invalid={isInvalidField("tax_id")}>
                                <div className="flex items-center w-full">
                                    <div className="flex flex-1 items-center">
                                        <FieldLabel htmlFor="tax_id">
                                            Adó
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
                                    value={taxId}
                                    onValueChange={(val) => {
                                        clearError("tax_id");
                                        setTaxId(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField("tax_id")}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {taxList.map((tax) => (
                                            <SelectItem
                                                key={tax.value}
                                                value={tax.value}
                                            >
                                                {tax.title}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.tax_id} />
                            </Field>
                            <Field data-invalid={isInvalidField("status")}>
                                <FieldLabel htmlFor="status">
                                    Státusz
                                </FieldLabel>
                                <Select
                                    value={status}
                                    onValueChange={(val) => {
                                        clearError("status");
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
                                        <SelectItem value="in_progress">
                                            Folyamatban
                                        </SelectItem>
                                        <SelectItem value="waiting_for_check">
                                            Bevizsgálásra vár
                                        </SelectItem>
                                        <SelectItem value="waiting_for_supply">
                                            Beszerzésre vár
                                        </SelectItem>
                                        <SelectItem value="waiting_for_feedback">
                                            Visszajelzésre vár
                                        </SelectItem>
                                        <SelectItem value="ready_for_customer_pickup">
                                            Átvehető
                                        </SelectItem>
                                        <SelectItem value="closed">
                                            Lezárt
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.status} />
                            </Field>
                            <Field data-invalid={isInvalidField("priority")}>
                                <FieldLabel htmlFor="priority">
                                    Prioritás
                                </FieldLabel>
                                <Select
                                    value={priority ?? ""}
                                    onValueChange={(val) => {
                                        clearError("priority");
                                        setPriority(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "priority",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="low">
                                            Alacsony
                                        </SelectItem>
                                        <SelectItem value="normal">
                                            Normál
                                        </SelectItem>
                                        <SelectItem value="high">
                                            Magas
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.priority} />
                            </Field>
                            <Field data-invalid={isInvalidField("due_date")}>
                                <FieldLabel htmlFor="due_date">
                                    Határidő
                                </FieldLabel>
                                <Input
                                    id="due_date"
                                    type="date"
                                    value={dueDate ?? ""}
                                    onChange={(e) => {
                                        clearError("due_date");
                                        setDueDate(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField("due_date")}
                                />
                                <FieldErrorV2 error={errors?.due_date} />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <Field orientation="horizontal">
                        <div className="text-right mt-8 w-full">
                            <Button
                                className="mr-3"
                                variant="outline"
                                onClick={(e: React.MouseEvent) => {
                                    e.preventDefault();
                                    navigate(-1);
                                }}
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
