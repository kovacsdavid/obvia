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

import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";
import { Input, FieldErrorV2, Button } from "@/components/ui";
import {
    type Address,
    type AddressErrors,
} from "@/components/modules/address/lib/interface";
import {
    Select,
    SelectContent,
    SelectItem,
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
import { useCallback, useEffect, useId } from "react";
import { useAppDispatch } from "@/store/hooks";
import { select_list } from "@/components/modules/address/lib/slice";
import { useSelectList } from "@/hooks/use_select_list.ts";
import React from "react";
import type { SelectOptionList } from "@/lib/interface";
import { useFormError } from "@/hooks/use_form_error";
import { RefreshCw } from "lucide-react";

interface AddressProps {
    value: Address | null | undefined;
    onChange: (next: Address) => void;
    errors?: AddressErrors | undefined;
    setErrors?: <T extends keyof AddressErrors>(
        value: string | null,
        field: T,
    ) => void;
    disabled?: boolean;
    label?: string;
}

const defaultAddressErrors = {
    id: null,
    type: null,
    country_code: null,
    postal_code: null,
    settlement: null,
    mailbox: null,
    topographic_number: null,
    name_of_public_space: null,
    type_of_public_space: null,
    house_number: null,
    building: null,
    stairway: null,
    floor: null,
    door: null,
};

export default function Address({
    value,
    onChange,
    errors = defaultAddressErrors,
    setErrors,
    disabled = false,
    label = "Cím",
}: AddressProps) {
    const id_prefix = useId();
    const dispatch = useAppDispatch();
    const { setListResponse } = useSelectList();
    const [countryList, setCountryList] = React.useState<SelectOptionList>([]);
    const { unexpectedError } = useFormError();
    const setField = <K extends keyof Address>(field: K, newValue: string) => {
        if (value !== null && typeof value !== "undefined") {
            onChange({
                ...value,
                [field]: newValue,
            });
        }
    };
    const setError = useCallback(
        <T extends keyof AddressErrors>(value: string | null, field: T) => {
            if (setErrors) {
                setErrors(value, field);
            }
        },
        [setErrors],
    );

    const loadLists = useCallback(async () => {
        return dispatch(select_list("countries")).then((response) => {
            if (select_list.fulfilled.match(response)) {
                if (response.payload.statusCode === 200) {
                    setListResponse(response.payload, setCountryList, () => {
                        if (
                            typeof response.payload.jsonData?.error !==
                            "undefined"
                        ) {
                            setError(
                                response.payload.jsonData.error.message ?? null,
                                "country_code",
                            );
                        }
                    });
                } else {
                    unexpectedError(response.payload.statusCode);
                }
            } else {
                unexpectedError();
            }
        });
    }, [dispatch, setError, setListResponse, unexpectedError]);

    useEffect(() => {
        loadLists();
    }, [dispatch, setErrors, unexpectedError, setListResponse, loadLists]);

    return (
        <>
            {value !== null && typeof value !== "undefined" ? (
                <FieldSet>
                    <FieldLegend>{label}</FieldLegend>
                    <FieldGroup>
                        <Field data-invalid={typeof errors?.type === "string"}>
                            <FieldLabel htmlFor={`${id_prefix}_type`}>
                                Típus
                            </FieldLabel>
                            <Select
                                value={value.type}
                                onValueChange={(val) => {
                                    setError(null, "type");
                                    setField("type", val);
                                }}
                                disabled={disabled}
                            >
                                <SelectTrigger
                                    id={`${id_prefix}_type`}
                                    className={"w-full"}
                                    aria-invalid={
                                        typeof errors?.type === "string"
                                    }
                                >
                                    <SelectValue />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="full_address">
                                        Teljes cím
                                    </SelectItem>
                                    <SelectItem value="topographic_number">
                                        Helyrajzi szám
                                    </SelectItem>
                                    <SelectItem value="mailbox">
                                        Postafiók
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                            <FieldErrorV2 error={errors?.type} />
                        </Field>

                        <Field
                            data-invalid={
                                typeof errors?.country_code === "string"
                            }
                        >
                            <FieldLabel htmlFor={`${id_prefix}_country_code`}>
                                Ország kód
                            </FieldLabel>
                            {/*
                                NOTE: it is necessary to render this select after the list is loaded
                                because otherwise onValueChange will be triggered with an empty string
                                and so the field will be empty when data loaded for update
                            */}
                            {countryList.length > 0 && (
                                <Combobox
                                    value={value.country_code}
                                    disabled={disabled}
                                    onValueChange={(val) => {
                                        setError(null, "country_code");
                                        setField("country_code", val ?? "");
                                    }}
                                    items={countryList.map((c) => c.value)}
                                    itemToStringLabel={(code) =>
                                        countryList.find(
                                            (c) => c.value === code,
                                        )?.title ?? ""
                                    }
                                >
                                    <ComboboxInput
                                        id={`${id_prefix}_country_code`}
                                        placeholder="Válassz országot"
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
                                                    {
                                                        countryList.find(
                                                            (c) =>
                                                                c.value ===
                                                                code,
                                                        )?.title
                                                    }
                                                </ComboboxItem>
                                            )}
                                        </ComboboxList>
                                    </ComboboxContent>
                                </Combobox>
                            )}
                            <FieldErrorV2 error={errors?.country_code} />
                            {typeof errors?.country_code === "string" ? (
                                <Button
                                    type="button"
                                    variant="outline"
                                    size="sm"
                                    className="w-fit"
                                    onClick={loadLists}
                                >
                                    <RefreshCw />
                                    Lista betöltése újra!
                                </Button>
                            ) : null}
                        </Field>

                        <Field
                            data-invalid={
                                typeof errors?.postal_code === "string"
                            }
                        >
                            <FieldLabel htmlFor={`${id_prefix}_postal_code`}>
                                Irányítószám
                            </FieldLabel>
                            <Input
                                id={`${id_prefix}_postal_code`}
                                type="text"
                                placeholder="1132"
                                value={value.postal_code}
                                onChange={(e) => {
                                    setError(null, "postal_code");
                                    setField("postal_code", e.target.value);
                                }}
                                aria-invalid={
                                    typeof errors?.postal_code === "string"
                                }
                                disabled={disabled}
                            />
                            <FieldErrorV2 error={errors?.postal_code} />
                        </Field>

                        <Field
                            data-invalid={
                                typeof errors?.settlement === "string"
                            }
                        >
                            <FieldLabel htmlFor={`${id_prefix}_settlement`}>
                                Település
                            </FieldLabel>
                            <Input
                                id={`${id_prefix}_settlement`}
                                type="text"
                                placeholder="Budapest"
                                value={value.settlement}
                                onChange={(e) => {
                                    setError(null, "settlement");
                                    setField("settlement", e.target.value);
                                }}
                                aria-invalid={
                                    typeof errors?.settlement === "string"
                                }
                                disabled={disabled}
                            />
                            <FieldErrorV2 error={errors?.settlement} />
                        </Field>

                        {value.type === "mailbox" ? (
                            <Field
                                data-invalid={
                                    typeof errors?.mailbox === "string"
                                }
                            >
                                <FieldLabel htmlFor={`${id_prefix}_mailbox`}>
                                    Postafiók
                                </FieldLabel>
                                <Input
                                    id={`${id_prefix}_mailbox`}
                                    type="text"
                                    placeholder="123"
                                    value={value.mailbox ?? ""}
                                    onChange={(e) => {
                                        setError(null, "mailbox");
                                        setField("mailbox", e.target.value);
                                    }}
                                    aria-invalid={
                                        typeof errors?.mailbox === "string"
                                    }
                                    disabled={disabled}
                                />
                                <FieldErrorV2 error={errors?.mailbox} />
                            </Field>
                        ) : null}

                        {value.type === "topographic_number" ? (
                            <Field
                                data-invalid={
                                    typeof errors?.topographic_number ===
                                    "string"
                                }
                            >
                                <FieldLabel
                                    htmlFor={`${id_prefix}_topographic_number`}
                                >
                                    Helyrajzi szám
                                </FieldLabel>
                                <Input
                                    id={`${id_prefix}_topographic_number`}
                                    type="text"
                                    placeholder="123/A"
                                    value={value.topographic_number ?? ""}
                                    onChange={(e) => {
                                        setError(null, "topographic_number");
                                        setField(
                                            "topographic_number",
                                            e.target.value,
                                        );
                                    }}
                                    aria-invalid={
                                        typeof errors?.topographic_number ===
                                        "string"
                                    }
                                    disabled={disabled}
                                />
                                <FieldErrorV2
                                    error={errors?.topographic_number}
                                />
                            </Field>
                        ) : null}

                        {value.type === "full_address" ? (
                            <>
                                <Field
                                    data-invalid={
                                        typeof errors?.name_of_public_space ===
                                        "string"
                                    }
                                >
                                    <FieldLabel
                                        htmlFor={`${id_prefix}_name_of_public_space`}
                                    >
                                        Közterület neve
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_name_of_public_space`}
                                        type="text"
                                        placeholder="Váci"
                                        value={value.name_of_public_space ?? ""}
                                        onChange={(e) => {
                                            setError(
                                                null,
                                                "name_of_public_space",
                                            );
                                            setField(
                                                "name_of_public_space",
                                                e.target.value,
                                            );
                                        }}
                                        aria-invalid={
                                            typeof errors?.name_of_public_space ===
                                            "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2
                                        error={errors?.name_of_public_space}
                                    />
                                </Field>

                                <Field
                                    data-invalid={
                                        typeof errors?.type_of_public_space ===
                                        "string"
                                    }
                                >
                                    <FieldLabel
                                        htmlFor={`${id_prefix}_type_of_public_space`}
                                    >
                                        Közterület neve
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_type_of_public_space`}
                                        type="text"
                                        placeholder="út"
                                        value={value.type_of_public_space ?? ""}
                                        onChange={(e) => {
                                            setError(
                                                null,
                                                "type_of_public_space",
                                            );
                                            setField(
                                                "type_of_public_space",
                                                e.target.value,
                                            );
                                        }}
                                        aria-invalid={
                                            typeof errors?.type_of_public_space ===
                                            "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2
                                        error={errors?.type_of_public_space}
                                    />
                                </Field>

                                <Field
                                    data-invalid={
                                        typeof errors?.house_number === "string"
                                    }
                                >
                                    <FieldLabel
                                        htmlFor={`${id_prefix}_house_number`}
                                    >
                                        Házszám
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_house_number`}
                                        type="text"
                                        placeholder="1234"
                                        value={value.house_number ?? ""}
                                        onChange={(e) => {
                                            setError(null, "house_number");
                                            setField(
                                                "house_number",
                                                e.target.value,
                                            );
                                        }}
                                        aria-invalid={
                                            typeof errors?.house_number ===
                                            "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2
                                        error={errors?.house_number}
                                    />
                                </Field>

                                <Field
                                    data-invalid={
                                        typeof errors?.building === "string"
                                    }
                                >
                                    <FieldLabel
                                        htmlFor={`${id_prefix}_building`}
                                    >
                                        Épület
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_building`}
                                        type="text"
                                        placeholder="A"
                                        value={value.building ?? ""}
                                        onChange={(e) => {
                                            setError(null, "building");
                                            setField(
                                                "building",
                                                e.target.value,
                                            );
                                        }}
                                        aria-invalid={
                                            typeof errors?.building === "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2 error={errors?.building} />
                                </Field>

                                <Field
                                    data-invalid={
                                        typeof errors?.stairway === "string"
                                    }
                                >
                                    <FieldLabel
                                        htmlFor={`${id_prefix}_stairway`}
                                    >
                                        Lépcsőház
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_stairway`}
                                        type="text"
                                        placeholder="1"
                                        value={value.stairway ?? ""}
                                        onChange={(e) => {
                                            setError(null, "stairway");
                                            setField(
                                                "stairway",
                                                e.target.value,
                                            );
                                        }}
                                        aria-invalid={
                                            typeof errors?.stairway === "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2 error={errors?.stairway} />
                                </Field>

                                <Field
                                    data-invalid={
                                        typeof errors?.floor === "string"
                                    }
                                >
                                    <FieldLabel htmlFor={`${id_prefix}_floor`}>
                                        Emelet
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_floor`}
                                        type="text"
                                        placeholder="2"
                                        value={value.floor ?? ""}
                                        onChange={(e) => {
                                            setError(null, "floor");
                                            setField("floor", e.target.value);
                                        }}
                                        aria-invalid={
                                            typeof errors?.floor === "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2 error={errors?.floor} />
                                </Field>

                                <Field
                                    data-invalid={
                                        typeof errors?.door === "string"
                                    }
                                >
                                    <FieldLabel htmlFor={`${id_prefix}_door`}>
                                        Ajtó
                                    </FieldLabel>
                                    <Input
                                        id={`${id_prefix}_door`}
                                        type="text"
                                        placeholder="3"
                                        value={value.door ?? ""}
                                        onChange={(e) => {
                                            setError(null, "door");
                                            setField("door", e.target.value);
                                        }}
                                        aria-invalid={
                                            typeof errors?.door === "string"
                                        }
                                        disabled={disabled}
                                    />
                                    <FieldErrorV2 error={errors?.door} />
                                </Field>
                            </>
                        ) : null}
                    </FieldGroup>
                </FieldSet>
            ) : null}
        </>
    );
}
