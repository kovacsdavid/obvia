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
import { Input, FieldErrorV2 } from "@/components/ui";
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
    const setField = <K extends keyof Address>(field: K, newValue: string) => {
        if (value !== null && typeof value !== "undefined") {
            onChange({
                ...value,
                [field]: newValue,
            });
        }
    };
    const setError = <T extends keyof AddressErrors>(
        value: string | null,
        field: T,
    ) => {
        if (setErrors) {
            setErrors(value, field);
        }
    };
    return (
        <>
            {value !== null && typeof value !== "undefined" ? (
                <FieldSet>
                    <FieldLegend>{label}</FieldLegend>
                    <FieldGroup>
                        <Field data-invalid={typeof errors?.type === "string"}>
                            <FieldLabel htmlFor="type">Típus</FieldLabel>
                            <Select
                                value={value.type}
                                onValueChange={(val) => {
                                    setError(null, "type");
                                    setField("type", val);
                                }}
                                disabled={disabled}
                            >
                                <SelectTrigger
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
                            <FieldLabel htmlFor="country_code">
                                Ország kód
                            </FieldLabel>
                            <Input
                                id="country_code"
                                type="text"
                                placeholder="HU"
                                value={value.country_code}
                                onChange={(e) => {
                                    setError(null, "country_code");
                                    setField("country_code", e.target.value);
                                }}
                                aria-invalid={
                                    typeof errors?.country_code === "string"
                                }
                                disabled={disabled}
                            />
                            <FieldErrorV2 error={errors?.country_code} />
                        </Field>

                        <Field
                            data-invalid={
                                typeof errors?.postal_code === "string"
                            }
                        >
                            <FieldLabel htmlFor="postal_code">
                                Irányítószám
                            </FieldLabel>
                            <Input
                                id="postal_code"
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
                            <FieldLabel htmlFor="settlement">
                                Település
                            </FieldLabel>
                            <Input
                                id="settlement"
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
                                <FieldLabel htmlFor="mailbox">
                                    Postafiók
                                </FieldLabel>
                                <Input
                                    id="mailbox"
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
                                <FieldLabel htmlFor="topographic_number">
                                    Helyrajzi szám
                                </FieldLabel>
                                <Input
                                    id="topographic_number"
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
                                    <FieldLabel htmlFor="name_of_public_space">
                                        Közterület neve
                                    </FieldLabel>
                                    <Input
                                        id="name_of_public_space"
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
                                    <FieldLabel htmlFor="type_of_public_space">
                                        Közterület neve
                                    </FieldLabel>
                                    <Input
                                        id="type_of_public_space"
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
                                    <FieldLabel htmlFor="house_number">
                                        Házszám
                                    </FieldLabel>
                                    <Input
                                        id="house_number"
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
                                    <FieldLabel htmlFor="building">
                                        Épület
                                    </FieldLabel>
                                    <Input
                                        id="building"
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
                                    <FieldLabel htmlFor="stairway">
                                        Lépcsőház
                                    </FieldLabel>
                                    <Input
                                        id="stairway"
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
                                    <FieldLabel htmlFor="floor">
                                        Emelet
                                    </FieldLabel>
                                    <Input
                                        id="floor"
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
                                    <FieldLabel htmlFor="door">Ajtó</FieldLabel>
                                    <Input
                                        id="door"
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
