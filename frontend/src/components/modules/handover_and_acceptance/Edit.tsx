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
import { useAuth } from "@/hooks/use_auth";
import {
    create,
    get_full,
    update,
} from "@/components/modules/handover_and_acceptance/lib/slice.ts";
import { select_list as selectHandoverList } from "@/components/modules/handover_and_acceptance/lib/slice.ts";
import type { Customer } from "@/components/modules/customers/lib/interface.ts";
import type {
    HandoverAndAcceptanceError,
    HandoverAndAcceptanceFull,
} from "./lib/interface.ts";
import type { SelectOptionList } from "@/lib/interface.ts";
import {
    formatReceiptTime,
    resolveReceiptTime,
    validReceiptTimeZone,
} from "./lib/receivedAt";
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
    const { claims } = useAuth();
    const params = useParams();
    const id = React.useMemo(() => params["id"] ?? null, [params]);
    const accessoryId = React.useId();

    const [customerId, setCustomerId] = React.useState("");
    const [customersList, setCustomersList] = React.useState<SelectOptionList>(
        [],
    );
    const [itemMakeId, setItemMakeId] = React.useState("");
    const [itemMakesList, setItemMakesList] = React.useState<SelectOptionList>(
        [],
    );
    const [newItemMake, setNewItemMake] = React.useState("");
    const [itemModelId, setItemModelId] = React.useState("");
    const [itemModelsList, setItemModelsList] =
        React.useState<SelectOptionList>([]);
    const [newItemModel, setNewItemModel] = React.useState("");
    const [itemIdentifierTypeId, setItemIdentifierTypeId] = React.useState("");
    const [itemIdentifierTypesList, setItemIdentifierTypesList] =
        React.useState<SelectOptionList>([]);
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
    const [defaultReceivedAt] = React.useState(() => new Date().toISOString());
    const [originalReceivedAt, setOriginalReceivedAt] = React.useState<
        string | null
    >(null);
    const [receivedAtChanged, setReceivedAtChanged] = React.useState(false);
    const [receivedAtOffset, setReceivedAtOffset] = React.useState("");
    const [receivedAtError, setReceivedAtError] = React.useState<string | null>(
        null,
    );
    const [errors, setErrors] =
        React.useState<HandoverAndAcceptanceError | null>(null);
    const [isSaving, setIsSaving] = React.useState(false);
    const savingRef = React.useRef(false);
    const [openNewCustomerDialog, setOpenNewCustomerDialog] =
        React.useState(false);

    const itemModelsRequestRef = React.useRef(0);
    const loadItemModels = useCallback(
        async (makeId: string) => {
            const requestId = ++itemModelsRequestRef.current;
            setItemModelsList([]);
            if (!makeId || makeId === "other") {
                return;
            }
            const result = await dispatch(
                selectHandoverList({ list: "item_models", param: makeId }),
            ).unwrap();
            handleApiResult(result, {
                onSuccess: ({ data }) => {
                    if (requestId === itemModelsRequestRef.current) {
                        setItemModelsList(data ?? []);
                    }
                },
            });
        },
        [dispatch],
    );

    const clearError = (field: keyof HandoverAndAcceptanceError) => {
        setErrors((prev) =>
            prev && prev[field] != null ? { ...prev, [field]: null } : prev,
        );
    };

    const isInvalidField = (field: keyof HandoverAndAcceptanceError) =>
        typeof errors?.[field] === "string";

    const loadLists = useCallback(async () => {
        await Promise.all([
            dispatch(selectHandoverList({ list: "customers" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => setCustomersList(data ?? []),
                    });
                }),
            dispatch(selectHandoverList({ list: "item_makes" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => setItemMakesList(data ?? []),
                    });
                }),
            dispatch(selectHandoverList({ list: "item_identifier_type" }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) =>
                            setItemIdentifierTypesList(data ?? []),
                    });
                }),
            dispatch(selectHandoverList({ list: "currencies" }))
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
                    onSuccess: async ({ data }) => {
                        if (!data) return;
                        if (data.item_make?.id) {
                            await loadItemModels(data.item_make.id);
                        } else {
                            setItemModelsList([]);
                        }
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
                        setOriginalReceivedAt(data.received_at);
                        setReceivedAtChanged(false);
                        setReceivedAtOffset("");
                    },
                });
            }
        });
    }, [dispatch, id, loadItemModels, loadLists]);

    const unchangedReceivedAt =
        originalReceivedAt ?? (id ? null : defaultReceivedAt);
    const receiptTimeValue =
        !receivedAtChanged && unchangedReceivedAt
            ? formatReceiptTime(unchangedReceivedAt, claims?.tz)
            : receivedAt;
    const receiptTimeResult = resolveReceiptTime(receiptTimeValue, claims?.tz);

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        if (savingRef.current) return;

        let receivedAtUtc: string;
        if (!validReceiptTimeZone(claims?.tz)) {
            setReceivedAtError("Az időzóna nincs megadva vagy érvénytelen.");
            return;
        }
        if (
            receiptTimeResult.kind === "invalid-time" ||
            receiptTimeResult.kind === "invalid-zone"
        ) {
            setReceivedAtError("Adjon meg egy érvényes dátumot és időpontot.");
            return;
        }
        if (receiptTimeResult.kind === "gap") {
            setReceivedAtError(
                "Ez az időpont az óraátállítás miatt nem létezik.",
            );
            return;
        }
        if (!receivedAtChanged && unchangedReceivedAt) {
            receivedAtUtc = unchangedReceivedAt;
        } else {
            const option =
                receiptTimeResult.options.length === 1
                    ? receiptTimeResult.options[0]
                    : receiptTimeResult.options.find(
                          (item) => item.offset === receivedAtOffset,
                      );
            if (!option) {
                setReceivedAtError(
                    "Válassza ki az időeltolást az ismétlődő időponthoz.",
                );
                return;
            }
            receivedAtUtc = option.utc;
        }
        setReceivedAtError(null);

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
            receivedAt: receivedAtUtc,
        };

        savingRef.current = true;
        setIsSaving(true);
        try {
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
        } finally {
            savingRef.current = false;
            setIsSaving(false);
        }
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
        setErrors((prev) =>
            prev?.included_accessories
                ? {
                      ...prev,
                      included_accessories: prev.included_accessories.map(
                          (error, errorIndex) =>
                              errorIndex === index ? null : error,
                      ),
                  }
                : prev,
        );
        setIncludedAccessories((prev) =>
            prev.map((item, itemIndex) => (itemIndex === index ? value : item)),
        );
    };

    const removeAccessory = (index: number) => {
        setIncludedAccessories((prev) => prev.filter((_, i) => i !== index));
        setErrors((prev) =>
            prev?.included_accessories
                ? {
                      ...prev,
                      included_accessories: prev.included_accessories.filter(
                          (_, i) => i !== index,
                      ),
                  }
                : prev,
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
                        createOnly
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
                                        setItemModelId("");
                                        setNewItemModel("");
                                        void loadItemModels(value);
                                    }}
                                >
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Nincs megadva" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {itemMakesList.map((make) => (
                                            <SelectItem
                                                key={make.value}
                                                value={make.value}
                                            >
                                                {make.title}
                                            </SelectItem>
                                        ))}
                                        {itemMakeId &&
                                            itemMakeId !== "other" &&
                                            !itemMakesList.some(
                                                (make) =>
                                                    make.value === itemMakeId,
                                            ) && (
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
                                        {itemModelsList.map((model) => (
                                            <SelectItem
                                                key={model.value}
                                                value={model.value}
                                            >
                                                {model.title}
                                            </SelectItem>
                                        ))}
                                        {itemModelId &&
                                            itemModelId !== "other" &&
                                            !itemModelsList.some(
                                                (model) =>
                                                    model.value === itemModelId,
                                            ) && (
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
                                        {itemIdentifierTypesList.map(
                                            (identifierType) => (
                                                <SelectItem
                                                    key={identifierType.value}
                                                    value={identifierType.value}
                                                >
                                                    {identifierType.title}
                                                </SelectItem>
                                            ),
                                        )}
                                        {itemIdentifierTypeId &&
                                            itemIdentifierTypeId !== "other" &&
                                            !itemIdentifierTypesList.some(
                                                (identifierType) =>
                                                    identifierType.value ===
                                                    itemIdentifierTypeId,
                                            ) && (
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
                                        aria-label="Tartozék hozzáadása"
                                        disabled={isSaving}
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
                                                <div className="min-w-0 flex-1 space-y-2">
                                                    <Input
                                                        aria-label={`Tartozék ${index + 1}`}
                                                        disabled={isSaving}
                                                        aria-invalid={
                                                            errors
                                                                ?.included_accessories?.[
                                                                index
                                                            ] != null
                                                        }
                                                        aria-describedby={
                                                            errors
                                                                ?.included_accessories?.[
                                                                index
                                                            ] != null
                                                                ? `${accessoryId}-error-${index}`
                                                                : undefined
                                                        }
                                                        value={accessory}
                                                        onChange={(e) =>
                                                            updateAccessory(
                                                                index,
                                                                e.target.value,
                                                            )
                                                        }
                                                    />
                                                    {errors
                                                        ?.included_accessories?.[
                                                        index
                                                    ] != null && (
                                                        <div
                                                            id={`${accessoryId}-error-${index}`}
                                                        >
                                                            <FieldErrorV2
                                                                error={
                                                                    errors
                                                                        .included_accessories[
                                                                        index
                                                                    ]
                                                                }
                                                            />
                                                        </div>
                                                    )}
                                                </div>
                                                <Button
                                                    type="button"
                                                    variant="outline"
                                                    aria-label={`Tartozék ${index + 1} törlése`}
                                                    disabled={isSaving}
                                                    onClick={() =>
                                                        removeAccessory(index)
                                                    }
                                                >
                                                    <Trash />
                                                </Button>
                                            </div>
                                        ),
                                    )}
                                </div>
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

                            <Field
                                data-invalid={
                                    !!receivedAtError ||
                                    isInvalidField("received_at")
                                }
                            >
                                <FieldLabel htmlFor="received_at">
                                    Átvétel dátuma és időpontja{" "}
                                    {claims?.tz ? `(${claims.tz})` : ""}
                                </FieldLabel>
                                <Input
                                    id="received_at"
                                    type="datetime-local"
                                    step="60"
                                    value={receiptTimeValue}
                                    onChange={(e) => {
                                        clearError("received_at");
                                        setReceivedAt(e.target.value);
                                        setReceivedAtChanged(true);
                                        setReceivedAtOffset("");
                                        setReceivedAtError(null);
                                    }}
                                    aria-invalid={
                                        !!receivedAtError ||
                                        isInvalidField("received_at")
                                    }
                                />
                                {receivedAtChanged &&
                                    receiptTimeResult.kind === "valid" &&
                                    receiptTimeResult.options.length > 1 && (
                                        <Field>
                                            <FieldLabel htmlFor="received_at_offset">
                                                Időeltolás
                                            </FieldLabel>
                                            <Select
                                                value={receivedAtOffset}
                                                onValueChange={(value) => {
                                                    setReceivedAtOffset(value);
                                                    setReceivedAtError(null);
                                                }}
                                            >
                                                <SelectTrigger
                                                    id="received_at_offset"
                                                    className="w-full"
                                                    aria-invalid={
                                                        !!receivedAtError
                                                    }
                                                >
                                                    <SelectValue placeholder="Válasszon időeltolást" />
                                                </SelectTrigger>
                                                <SelectContent>
                                                    {receiptTimeResult.options.map(
                                                        (option) => (
                                                            <SelectItem
                                                                key={
                                                                    option.offset
                                                                }
                                                                value={
                                                                    option.offset
                                                                }
                                                            >
                                                                UTC
                                                                {option.offset}
                                                            </SelectItem>
                                                        ),
                                                    )}
                                                </SelectContent>
                                            </Select>
                                        </Field>
                                    )}
                                <FieldErrorV2
                                    error={
                                        receivedAtError ?? errors?.received_at
                                    }
                                />
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
                            <Button type="submit" disabled={isSaving}>
                                {id ? "Módosítás" : "Létrehozás"}
                            </Button>
                        </div>
                    </Field>
                </form>
            </ConditionalCard>
        </>
    );
}
