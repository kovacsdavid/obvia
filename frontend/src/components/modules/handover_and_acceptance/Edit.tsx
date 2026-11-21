/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
import { useNavigate, useParams } from "react-router";
import { Plus, Trash } from "lucide-react";
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { Textarea } from "@/components/ui/textarea.tsx";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import CustomersEdit from "@/components/modules/customers/Edit.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    create,
    get_full,
    update,
} from "@/components/modules/handover_and_acceptance/lib/slice.ts";
import { list as listCustomers } from "@/components/modules/customers/lib/slice.ts";
import { select_list as selectList } from "@/components/modules/tasks/lib/slice.ts";
import type { Customer } from "@/components/modules/customers/lib/interface.ts";
import type {
    HandoverAndAcceptanceError,
    HandoverAndAcceptanceFull,
} from "./lib/interface.ts";
import type { SelectOptionList } from "@/lib/interface.ts";
import { formatDateToYMD } from "@/lib/utils.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";

interface EditProps {
    showCard?: boolean;
    onSuccess?: (handover: HandoverAndAcceptanceFull) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    onSuccess,
    onCancel,
}: EditProps) {
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const params = useParams();
    const id = React.useMemo(() => params["id"] ?? null, [params]);

    const [customerId, setCustomerId] = React.useState("");
    const [customersList, setCustomersList] = React.useState<SelectOptionList>(
        [],
    );
    const [itemMakeId, setItemMakeId] = React.useState("");
    const [newItemMake, setNewItemMake] = React.useState("");
    const [itemModelId, setItemModelId] = React.useState("");
    const [newItemModel, setNewItemModel] = React.useState("");
    const [itemIdentifierTypeId, setItemIdentifierTypeId] = React.useState("");
    const [newItemIdentifierType, setNewItemIdentifierType] =
        React.useState("");
    const [itemIdentifierValue, setItemIdentifierValue] = React.useState("");
    const [reportedIssue, setReportedIssue] = React.useState("");
    const [conditionAtReceipt, setConditionAtReceipt] = React.useState("");
    const [includedAccessories, setIncludedAccessories] = React.useState<
        string[]
    >([]);
    const [authorizedWork, setAuthorizedWork] = React.useState("");
    const [repairLimit, setRepairLimit] = React.useState("");
    const [currencyCode, setCurrencyCode] = React.useState("HUF");
    const [currencyList, setCurrencyList] = React.useState<SelectOptionList>(
        [],
    );
    const [termsVersion, setTermsVersion] = React.useState("");
    const [receivedAt, setReceivedAt] = React.useState("");
    const [errors, setErrors] =
        React.useState<HandoverAndAcceptanceError | null>(null);
    const [openNewCustomerDialog, setOpenNewCustomerDialog] =
        React.useState(false);

    const clearError = (field: keyof HandoverAndAcceptanceError) => {
        setErrors((prev) =>
            prev && prev[field] != null ? { ...prev, [field]: null } : prev,
        );
    };

    const isInvalidField = (field: keyof HandoverAndAcceptanceError) =>
        typeof errors?.[field] === "string";

    const loadLists = useCallback(async () => {
        await Promise.all([
            dispatch(listCustomers({ query: null }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            setCustomersList(
                                data?.map((customer) => ({
                                    value: customer.id,
                                    title: customer.name,
                                })) ?? [],
                            );
                        },
                    });
                }),
            dispatch(selectList({ list: "currencies" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => setCurrencyList(data ?? []),
                    });
                }),
        ]);
    }, [dispatch]);

    useEffect(() => {
        loadLists().then(async () => {
            if (typeof id === "string") {
                const result = await dispatch(get_full({ uuid: id })).unwrap();
                handleApiResult(result, {
                    onSuccess: ({ data }) => {
                        if (!data) return;
                        setCustomerId(data.customer.id);
                        setItemMakeId(data.item_make?.id ?? "");
                        setNewItemMake("");
                        setItemModelId(data.item_model?.id ?? "");
                        setNewItemModel("");
                        setItemIdentifierTypeId(data.item_identifier_type.id);
                        setNewItemIdentifierType("");
                        setItemIdentifierValue(data.item_identifier_value);
                        setReportedIssue(data.reported_issue ?? "");
                        setConditionAtReceipt(data.condition_at_receipt ?? "");
                        setIncludedAccessories(data.included_accessories ?? []);
                        setAuthorizedWork(data.authorized_work ?? "");
                        setRepairLimit(data.repair_limit ?? "");
                        setCurrencyCode(data.currency_code?.code ?? "");
                        setTermsVersion(data.terms_version ?? "");
                        setReceivedAt(formatDateToYMD(data.received_at));
                    },
                });
            }
        });
    }, [dispatch, id, loadLists]);

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();

        const input = {
            id,
            customerId,
            itemMakeId,
            newItemMake,
            itemModelId,
            newItemModel,
            itemIdentifierTypeId,
            newItemIdentifierType,
            itemIdentifierValue,
            reportedIssue,
            conditionAtReceipt,
            includedAccessories,
            authorizedWork,
            repairLimit,
            currencyCode,
            termsVersion,
            receivedAt,
        };

        const result = await dispatch(
            id ? update({ input }) : create({ input }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: ({ data }) => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                if (typeof onSuccess === "function" && data) {
                    onSuccess(data as unknown as HandoverAndAcceptanceFull);
                } else {
                    navigate("/atadas-atvetel/lista");
                }
            },
            onFieldErrors: setErrors,
        });
    };

    const handleCancel = (e: React.MouseEvent) => {
        e.preventDefault();
        if (typeof onCancel === "function") onCancel();
        else navigate(-1);
    };

    const handleEditCustomersSuccess = (customer: Customer) => {
        loadLists().then(() => {
            setCustomerId(customer.id);
            setOpenNewCustomerDialog(false);
        });
    };

    const updateAccessory = (index: number, value: string) => {
        clearError("included_accessories");
        setIncludedAccessories((prev) =>
            prev.map((item, itemIndex) => (itemIndex === index ? value : item)),
        );
    };

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
            <ConditionalCard showCard={showCard} className="max-w-3xl mx-auto">
                <form
                    onSubmit={handleSubmit}
                    className="space-y-4"
                    autoComplete="off"
                >
                    <FieldSet>
                        <FieldLegend>
                            {`Átadás-átvétel ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            <Field data-invalid={isInvalidField("customer_id")}>
                                <div className="flex items-center w-full">
                                    <div className="flex flex-1 items-center">
                                        <FieldLabel htmlFor="customer_id">
                                            Vevő
                                        </FieldLabel>
                                    </div>
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
                                <Select
                                    value={customerId}
                                    onValueChange={(value) => {
                                        clearError("customer_id");
                                        setCustomerId(value);
                                    }}
                                >
                                    <SelectTrigger
                                        className="w-full"
                                        aria-invalid={isInvalidField(
                                            "customer_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {customersList.map((customer) => (
                                            <SelectItem
                                                key={customer.value}
                                                value={customer.value}
                                            >
                                                {customer.title}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.customer_id} />
                            </Field>

                            <Field
                                data-invalid={
                                    isInvalidField("item_make_id") ||
                                    isInvalidField("new_item_make")
                                }
                            >
                                <FieldLabel htmlFor="item_make_id">
                                    Tárgy gyártója
                                </FieldLabel>
                                <Select
                                    value={itemMakeId === "" ? "" : itemMakeId}
                                    onValueChange={(value) => {
                                        clearError("item_make_id");
                                        clearError("new_item_make");
                                        setItemMakeId(value);
                                    }}
                                >
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Nincs megadva" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {itemMakeId && (
                                            <SelectItem value={itemMakeId}>
                                                Meglévő gyártó
                                            </SelectItem>
                                        )}
                                        <SelectItem value="other">
                                            Új gyártó
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                {itemMakeId === "other" && (
                                    <Input
                                        value={newItemMake}
                                        onChange={(e) => {
                                            clearError("new_item_make");
                                            setNewItemMake(e.target.value);
                                        }}
                                        placeholder="Gyártó neve"
                                    />
                                )}
                                <FieldErrorV2
                                    error={
                                        errors?.item_make_id ??
                                        errors?.new_item_make
                                    }
                                />
                            </Field>

                            <Field
                                data-invalid={
                                    isInvalidField("item_model_id") ||
                                    isInvalidField("new_item_model")
                                }
                            >
                                <FieldLabel htmlFor="item_model_id">
                                    Tárgy típusa
                                </FieldLabel>
                                <Select
                                    value={
                                        itemModelId === "" ? "" : itemModelId
                                    }
                                    onValueChange={(value) => {
                                        clearError("item_model_id");
                                        clearError("new_item_model");
                                        setItemModelId(value);
                                    }}
                                >
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Nincs megadva" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {itemModelId && (
                                            <SelectItem value={itemModelId}>
                                                Meglévő típus
                                            </SelectItem>
                                        )}
                                        <SelectItem value="other">
                                            Új típus
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                {itemModelId === "other" && (
                                    <Input
                                        value={newItemModel}
                                        onChange={(e) => {
                                            clearError("new_item_model");
                                            setNewItemModel(e.target.value);
                                        }}
                                        placeholder="Típus neve"
                                    />
                                )}
                                <FieldErrorV2
                                    error={
                                        errors?.item_model_id ??
                                        errors?.new_item_model
                                    }
                                />
                            </Field>

                            <Field
                                data-invalid={
                                    isInvalidField("item_identifier_type_id") ||
                                    isInvalidField("new_item_identifier_type")
                                }
                            >
                                <FieldLabel htmlFor="item_identifier_type_id">
                                    Azonosító típusa
                                </FieldLabel>
                                <Select
                                    value={
                                        itemIdentifierTypeId === ""
                                            ? ""
                                            : itemIdentifierTypeId
                                    }
                                    onValueChange={(value) => {
                                        clearError("item_identifier_type_id");
                                        clearError("new_item_identifier_type");
                                        setItemIdentifierTypeId(value);
                                    }}
                                >
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Válasszon" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {itemIdentifierTypeId && (
                                            <SelectItem
                                                value={itemIdentifierTypeId}
                                            >
                                                Meglévő típus
                                            </SelectItem>
                                        )}
                                        <SelectItem value="other">
                                            Új azonosító típus
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                {itemIdentifierTypeId === "other" && (
                                    <Input
                                        value={newItemIdentifierType}
                                        onChange={(e) => {
                                            clearError(
                                                "new_item_identifier_type",
                                            );
                                            setNewItemIdentifierType(
                                                e.target.value,
                                            );
                                        }}
                                        placeholder="Azonosító típus neve"
                                    />
                                )}
                                <FieldErrorV2
                                    error={
                                        errors?.item_identifier_type_id ??
                                        errors?.new_item_identifier_type
                                    }
                                />
                            </Field>

                            <Field
                                data-invalid={isInvalidField(
                                    "item_identifier_value",
                                )}
                            >
                                <FieldLabel htmlFor="item_identifier_value">
                                    Azonosító értéke
                                </FieldLabel>
                                <Input
                                    id="item_identifier_value"
                                    value={itemIdentifierValue}
                                    onChange={(e) => {
                                        clearError("item_identifier_value");
                                        setItemIdentifierValue(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField(
                                        "item_identifier_value",
                                    )}
                                />
                                <FieldErrorV2
                                    error={errors?.item_identifier_value}
                                />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("reported_issue")}
                            >
                                <FieldLabel htmlFor="reported_issue">
                                    Bejelentett probléma
                                </FieldLabel>
                                <Textarea
                                    id="reported_issue"
                                    value={reportedIssue}
                                    onChange={(e) => {
                                        clearError("reported_issue");
                                        setReportedIssue(e.target.value);
                                    }}
                                />
                                <FieldErrorV2 error={errors?.reported_issue} />
                            </Field>

                            <Field
                                data-invalid={isInvalidField(
                                    "condition_at_receipt",
                                )}
                            >
                                <FieldLabel htmlFor="condition_at_receipt">
                                    Állapot átvételkor
                                </FieldLabel>
                                <Textarea
                                    id="condition_at_receipt"
                                    value={conditionAtReceipt}
                                    onChange={(e) => {
                                        clearError("condition_at_receipt");
                                        setConditionAtReceipt(e.target.value);
                                    }}
                                />
                                <FieldErrorV2
                                    error={errors?.condition_at_receipt}
                                />
                            </Field>

                            <Field>
                                <div className="flex items-center justify-between">
                                    <FieldLabel>
                                        Mellékelt tartozékok
                                    </FieldLabel>
                                    <Button
                                        type="button"
                                        variant="outline"
                                        onClick={() =>
                                            setIncludedAccessories((prev) => [
                                                ...prev,
                                                "",
                                            ])
                                        }
                                    >
                                        <Plus />
                                    </Button>
                                </div>
                                <div className="space-y-2">
                                    {includedAccessories.map(
                                        (accessory, index) => (
                                            <div
                                                key={index}
                                                className="flex gap-2"
                                            >
                                                <Input
                                                    value={accessory}
                                                    onChange={(e) =>
                                                        updateAccessory(
                                                            index,
                                                            e.target.value,
                                                        )
                                                    }
                                                />
                                                <Button
                                                    type="button"
                                                    variant="outline"
                                                    onClick={() =>
                                                        setIncludedAccessories(
                                                            (prev) =>
                                                                prev.filter(
                                                                    (_, i) =>
                                                                        i !==
                                                                        index,
                                                                ),
                                                        )
                                                    }
                                                >
                                                    <Trash />
                                                </Button>
                                            </div>
                                        ),
                                    )}
                                </div>
                                <FieldErrorV2
                                    error={errors?.included_accessories?.[0]}
                                />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("authorized_work")}
                            >
                                <FieldLabel htmlFor="authorized_work">
                                    Engedélyezett munka
                                </FieldLabel>
                                <Textarea
                                    id="authorized_work"
                                    value={authorizedWork}
                                    onChange={(e) => {
                                        clearError("authorized_work");
                                        setAuthorizedWork(e.target.value);
                                    }}
                                />
                                <FieldErrorV2 error={errors?.authorized_work} />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("repair_limit")}
                            >
                                <FieldLabel htmlFor="repair_limit">
                                    Javítási keret
                                </FieldLabel>
                                <Input
                                    id="repair_limit"
                                    value={repairLimit}
                                    onChange={(e) => {
                                        clearError("repair_limit");
                                        setRepairLimit(e.target.value);
                                    }}
                                />
                                <FieldErrorV2 error={errors?.repair_limit} />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("currency_code")}
                            >
                                <FieldLabel htmlFor="currency_code">
                                    Pénznem
                                </FieldLabel>
                                <Select
                                    value={currencyCode}
                                    onValueChange={(value) => {
                                        clearError("currency_code");
                                        setCurrencyCode(value);
                                    }}
                                >
                                    <SelectTrigger className="w-full">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {currencyList.map((currency) => (
                                            <SelectItem
                                                key={currency.value}
                                                value={currency.value}
                                            >
                                                {currency.title}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.currency_code} />
                            </Field>

                            <Field
                                data-invalid={isInvalidField("terms_version")}
                            >
                                <FieldLabel htmlFor="terms_version">
                                    Feltétel verzió
                                </FieldLabel>
                                <Input
                                    id="terms_version"
                                    value={termsVersion}
                                    onChange={(e) => {
                                        clearError("terms_version");
                                        setTermsVersion(e.target.value);
                                    }}
                                />
                                <FieldErrorV2 error={errors?.terms_version} />
                            </Field>

                            <Field data-invalid={isInvalidField("received_at")}>
                                <FieldLabel htmlFor="received_at">
                                    Átvétel dátuma
                                </FieldLabel>
                                <Input
                                    id="received_at"
                                    type="date"
                                    value={receivedAt}
                                    onChange={(e) => {
                                        clearError("received_at");
                                        setReceivedAt(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField("received_at")}
                                />
                                <FieldErrorV2 error={errors?.received_at} />
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
