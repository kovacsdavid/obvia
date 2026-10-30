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
} from "@/components/modules/worksheets/lib/slice.ts";
import { type SelectOptionList } from "@/lib/interface.ts";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { useNavigate } from "react-router";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { Plus } from "lucide-react";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import CustomersEdit from "@/components/modules/customers/Edit.tsx";
import type { Worksheet, WorksheetError } from "./lib/interface";
import type { Customer } from "@/components/modules/customers/lib/interface.ts";
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
    onSuccess?: (worksheet: Worksheet) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [description, setDescription] = React.useState("");
    const [customerId, setCustomerId] = React.useState("");
    const [status, setStatus] = React.useState("active");
    const [customersList, setCustomersList] = React.useState<SelectOptionList>(
        [],
    );
    const [errors, setErrors] = React.useState<WorksheetError | null>(null);
    const [openNewCustomerDialog, setOpenNewCustomerDialog] =
        React.useState(false);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const { setListResponse } = useSelectList();
    const params = useParams();
    const id = React.useMemo(() => params["id"] ?? null, [params]);

    const handleCreate = async () => {
        const result = await dispatch(
            create({
                input: {
                    id,
                    description,
                    customerId,
                    projectId: "",
                    status,
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
                    navigate("/munkalap/lista");
                }
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
        const result = await dispatch(
            update({
                input: {
                    id,
                    description,
                    customerId,
                    projectId: "",
                    status,
                },
            }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/munkalap/lista");
            },
            onFieldErrors: setErrors,
        });
    };

    const loadLists = useCallback(async () => {
        return Promise.all([
            dispatch(select_list({ list: "customers" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setCustomersList(data);
                            } else {
                                setCustomersList([]);
                            }
                        },
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
                            setDescription(data.description ?? "");
                            setCustomerId(data.customer_id);
                            setStatus(data.status ?? "");
                        }
                    },
                });
            }
        });
    }, [dispatch, id, setErrors, setListResponse, loadLists]);

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        if (typeof id === "string") {
            handleUpdate();
        } else {
            handleCreate();
        }
    };

    const handleEditCustomersSuccess = (customer: Customer) => {
        loadLists().then(() => {
            setTimeout(() => {
                setCustomerId(customer.id);
            }, 0);
            setOpenNewCustomerDialog(false);
        });
    };

    const clearError = (field: keyof WorksheetError) => {
        setErrors((prev) =>
            prev && prev[field] != null ? { ...prev, [field]: null } : prev,
        );
    };

    const isInvalidField = (field: keyof WorksheetError) =>
        typeof errors?.[field] === "string";

    return (
        <>
            <Dialog
                open={openNewCustomerDialog}
                onOpenChange={setOpenNewCustomerDialog}
            >
                <DialogContent>
                    <DialogTitle>Vevő létrehozása</DialogTitle>
                    <CustomersEdit
                        showCard={false}
                        onSuccess={handleEditCustomersSuccess}
                        onCancel={() => setOpenNewCustomerDialog(false)}
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
                            {`Munkalap ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
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
                            <Field data-invalid={isInvalidField("customer_id")}>
                                <div className="flex items-center w-full">
                                    <div className="flex flex-1 items-center">
                                        <FieldLabel htmlFor="customer_id">
                                            Vevő
                                        </FieldLabel>
                                    </div>
                                    <div className="flex items-center">
                                        <Button
                                            type="button"
                                            variant="outline"
                                            onClick={() =>
                                                setOpenNewCustomerDialog(true)
                                            }
                                        >
                                            <Plus />
                                        </Button>
                                    </div>
                                </div>
                                <Select
                                    value={customerId}
                                    onValueChange={(val) => {
                                        clearError("customer_id");
                                        setCustomerId(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "customer_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {customersList.map((customer) => {
                                            return (
                                                <SelectItem
                                                    key={customer.value}
                                                    value={customer.value}
                                                >
                                                    {customer.title}
                                                </SelectItem>
                                            );
                                        })}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.customer_id} />
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
                                        <SelectItem value="open">
                                            Nyitott
                                        </SelectItem>
                                        <SelectItem value="closed">
                                            Lezárt
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
