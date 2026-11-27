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

import React, { useCallback, useEffect, useMemo } from "react";
import type { z } from "zod";
import type { taxFormSchema } from "./lib/schemas";
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    create,
    get,
    select_list,
    update,
} from "@/components/modules/taxes/lib/slice.ts";
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectSeparator,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import {
    Combobox,
    ComboboxContent,
    ComboboxEmpty,
    ComboboxInput,
    ComboboxItem,
    ComboboxList,
} from "@/components/ui/combobox";
import { useNavigate } from "react-router";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { SelectOptionList } from "@/lib/interface.ts";
import type { Tax, TaxError } from "./lib/interface";
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
    onSuccess?: (tax: Tax) => void;
    onCancel?: () => void;
}

type TaxFormState = z.infer<typeof taxFormSchema>;

export default function Edit({
    showCard = true,
    createOnly = false,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [form, setForm] = React.useState<TaxFormState>({
        rate: "",
        description: "",
        country_code: "HU",
        tax_category: "",
        is_rate_applicable: "",
        legal_text: "",
        reporting_code: "",
        is_default: false,
        status: "active",
    });
    const [countryList, setCountryList] = React.useState<SelectOptionList>([]);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const [errors, setErrors] = React.useState<TaxError | null>(null);
    const params = useParams();
    const id = createOnly ? null : (params["id"] ?? null);

    function setFormField<Key extends keyof TaxFormState>(
        field: Key,
        value: TaxFormState[Key],
    ) {
        setForm((previous) => ({ ...previous, [field]: value }));
    }

    const evalIsRateApplicable = (value: string): boolean | null => {
        switch (value) {
            case "true":
                return true;
            case "false":
                return false;
            default:
                return null;
        }
    };

    const handleCreate = async () => {
        const result = await dispatch(
            create({
                input: {
                    ...form,
                    id,
                    is_rate_applicable: evalIsRateApplicable(
                        form.is_rate_applicable,
                    ),
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
                )
                    onSuccess(data);
                else navigate("/ado/lista");
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
                    ...form,
                    id,
                    is_rate_applicable: evalIsRateApplicable(
                        form.is_rate_applicable,
                    ),
                },
            }),
        ).unwrap();
        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/ado/lista");
            },
            onFieldErrors: setErrors,
        });
    };

    const loadLists = useCallback(async () => {
        const result = await dispatch(
            select_list({ list: "countries" }),
        ).unwrap();
        handleApiResult(result, {
            onSuccess: ({ data }) => setCountryList(data ?? []),
        });
    }, [dispatch]);

    useEffect(() => {
        loadLists()
            .then(async () => {
                if (typeof id === "string") {
                    const result = await dispatch(get({ uuid: id })).unwrap();
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setForm({
                                    rate: data.rate ?? "",
                                    description: data.description,
                                    country_code: data.country_code,
                                    tax_category: data.tax_category,
                                    is_rate_applicable: data.is_rate_applicable
                                        ? "true"
                                        : "false",
                                    legal_text: data.legal_text ?? "",
                                    reporting_code: data.reporting_code ?? "",
                                    is_default: data.is_default,
                                    status: data.status,
                                });
                            }
                        },
                    });
                }
            })
            .catch(() => undefined);
    }, [dispatch, id, loadLists]);

    const resetError = (field: keyof TaxError) => {
        setErrors((previous) =>
            previous && previous[field] != null
                ? { ...previous, [field]: null }
                : previous,
        );
    };
    const isInvalidField = (field: keyof TaxError) =>
        typeof errors?.[field] === "string";

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        if (typeof id === "string") {
            handleUpdate();
        } else {
            handleCreate();
        }
    };

    const countryTitleByCode = useMemo(
        () => new Map(countryList.map((c) => [c.value, c.title])),
        [countryList],
    );

    const countryCodes = useMemo(
        () => countryList.map((c) => c.value),
        [countryList],
    );

    const getCountryTitle = useCallback(
        (code: string) => countryTitleByCode.get(code) ?? "",
        [countryTitleByCode],
    );

    return (
        <>
            <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
                <form
                    onSubmit={handleSubmit}
                    className="space-y-4"
                    autoComplete={"off"}
                >
                    <FieldSet>
                        <FieldLegend>
                            {`Adók ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            <Field
                                data-invalid={isInvalidField(
                                    "is_rate_applicable",
                                )}
                            >
                                <FieldLabel htmlFor="is_rate_applicable">
                                    Adókulcs alkalmazandó
                                </FieldLabel>
                                <Select
                                    value={form.is_rate_applicable}
                                    onValueChange={(val) => {
                                        resetError("is_rate_applicable");
                                        setFormField("is_rate_applicable", val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "is_rate_applicable",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value={"true"}>
                                            igen
                                        </SelectItem>
                                        <SelectItem value={"false"}>
                                            nem
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2
                                    error={errors?.is_rate_applicable}
                                />
                            </Field>
                            {form.is_rate_applicable === "true" ? (
                                <>
                                    <Field
                                        data-invalid={isInvalidField("rate")}
                                    >
                                        <FieldLabel htmlFor="rate">
                                            Adókulcs (%)
                                        </FieldLabel>
                                        <Input
                                            id="rate"
                                            type="text"
                                            value={form.rate}
                                            onChange={(e) => {
                                                resetError("rate");
                                                setFormField(
                                                    "rate",
                                                    e.target.value,
                                                );
                                            }}
                                            aria-invalid={isInvalidField(
                                                "rate",
                                            )}
                                        />
                                        <FieldErrorV2 error={errors?.rate} />
                                    </Field>
                                </>
                            ) : null}
                            <Field data-invalid={isInvalidField("description")}>
                                <FieldLabel htmlFor="description">
                                    Megnevezés
                                </FieldLabel>
                                <Input
                                    id="description"
                                    type="text"
                                    placeholder="ÁFA"
                                    value={form.description}
                                    onChange={(e) => {
                                        resetError("description");
                                        setFormField(
                                            "description",
                                            e.target.value,
                                        );
                                    }}
                                    aria-invalid={isInvalidField("description")}
                                />
                                <FieldErrorV2 error={errors?.description} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("country_code")}
                            >
                                <FieldLabel htmlFor="country_code">
                                    Ország
                                </FieldLabel>
                                <Combobox
                                    value={form.country_code}
                                    onValueChange={(val) => {
                                        resetError("country_code");
                                        setFormField("country_code", val ?? "");
                                    }}
                                    items={countryCodes}
                                    itemToStringLabel={getCountryTitle}
                                >
                                    <ComboboxInput
                                        id={"country_code"}
                                        placeholder="Válassz országot"
                                        aria-invalid={isInvalidField(
                                            "country_code",
                                        )}
                                    />
                                    <ComboboxContent>
                                        <ComboboxEmpty>
                                            Nincs találat.
                                        </ComboboxEmpty>
                                        <ComboboxList>
                                            {(code: string) => (
                                                <ComboboxItem
                                                    key={code}
                                                    value={code}
                                                >
                                                    {getCountryTitle(code)}
                                                </ComboboxItem>
                                            )}
                                        </ComboboxList>
                                    </ComboboxContent>
                                </Combobox>
                                <FieldErrorV2 error={errors?.country_code} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("tax_category")}
                            >
                                <FieldLabel htmlFor="tax_category">
                                    Adó kategória
                                </FieldLabel>
                                <Select
                                    value={form.tax_category}
                                    onValueChange={(val) => {
                                        resetError("tax_category");
                                        setFormField("tax_category", val);
                                    }}
                                >
                                    <SelectTrigger
                                        id="tax_category"
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "tax_category",
                                        )}
                                    >
                                        <SelectValue placeholder="Válasszon adó kategóriát" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectGroup>
                                            <SelectLabel>
                                                Adóköteles ügyletek
                                            </SelectLabel>
                                            <SelectItem value="AK">
                                                Általános adókulcs
                                            </SelectItem>
                                        </SelectGroup>
                                        <SelectSeparator />
                                        <SelectGroup>
                                            <SelectLabel>
                                                Adómentes ügyletek
                                            </SelectLabel>
                                            <SelectItem value="AAM">
                                                Alanyi adómentes (AAM)
                                            </SelectItem>
                                            <SelectItem value="TAM">
                                                Tárgyi adómentes (TAM)
                                            </SelectItem>
                                            <SelectItem value="KBAET">
                                                Közösségen belüli adómentes
                                                termékértékesítés (KBAET)
                                            </SelectItem>
                                            <SelectItem value="KBAUK">
                                                Új közlekedési eszköz Közösségen
                                                belüli értékesítése (KBAUK)
                                            </SelectItem>
                                            <SelectItem value="EAM">
                                                Adómentes termékexport (harmadik
                                                országba) (EAM)
                                            </SelectItem>
                                            <SelectItem value="NAM">
                                                Nemzetközi ügyletekhez
                                                kapcsolódó adómentesség (NAM)
                                            </SelectItem>
                                        </SelectGroup>
                                        <SelectSeparator />
                                        <SelectGroup>
                                            <SelectLabel>
                                                Áfa tárgyi hatályán kívül
                                            </SelectLabel>
                                            <SelectItem value="ATK">
                                                Áfa tárgyi hatályán kívüli (ATK)
                                            </SelectItem>
                                            <SelectItem value="EUFAD37">
                                                Másik tagállamban teljesített
                                                szolgáltatás (37. § (1))
                                                (EUFAD37)
                                            </SelectItem>
                                            <SelectItem value="EUFADE">
                                                Másik tagállamban teljesített,
                                                egyéb fordítottan adózó ügylet
                                                (EUFADE)
                                            </SelectItem>
                                            <SelectItem value="EUE">
                                                Másik tagállamban teljesített,
                                                nem fordítottan adózó ügylet
                                                (EUE)
                                            </SelectItem>
                                            <SelectItem value="HO">
                                                Harmadik országban teljesített
                                                ügylet (HO)
                                            </SelectItem>
                                        </SelectGroup>
                                        <SelectSeparator />
                                        <SelectGroup>
                                            <SelectLabel>
                                                Ingyenes, ellenérték fejében
                                                végzettnek minősülő ügylet
                                            </SelectLabel>
                                            <SelectItem value="REFUNDABLE_VAT">
                                                Áfát a kedvezményezett megtéríti
                                                (REFUNDABLE_VAT)
                                            </SelectItem>
                                            <SelectItem value="NONREFUNDABLE_VAT">
                                                Áfát a kedvezményezett nem
                                                téríti meg (NONREFUNDABLE_VAT)
                                            </SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.tax_category} />
                            </Field>
                            <Field data-invalid={isInvalidField("legal_text")}>
                                <FieldLabel htmlFor="legal_text">
                                    Jogi szöveg
                                </FieldLabel>
                                <Input
                                    id="legal_text"
                                    type="text"
                                    placeholder="Pl.: Alanyi adómenetes"
                                    value={form.legal_text}
                                    onChange={(e) => {
                                        resetError("legal_text");
                                        setFormField(
                                            "legal_text",
                                            e.target.value,
                                        );
                                    }}
                                    aria-invalid={isInvalidField("legal_text")}
                                />
                                <FieldErrorV2 error={errors?.legal_text} />
                            </Field>
                            <Field data-invalid={isInvalidField("status")}>
                                <FieldLabel htmlFor="status">
                                    Státusz
                                </FieldLabel>
                                <Select
                                    value={form.status}
                                    onValueChange={(val) => {
                                        resetError("status");
                                        setFormField("status", val);
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
                                        <SelectItem value="draft">
                                            Vázlat
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
