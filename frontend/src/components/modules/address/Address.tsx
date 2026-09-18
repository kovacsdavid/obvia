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

import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field";
import { FieldError, GlobalError, Input } from "@/components/ui";
import { type Address, type AddressErrors } from "@/components/modules/address/lib/interface";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";

interface AddressProps {
    value: Address;
    onChange: (next: Address) => void;
    errors?: AddressErrors;
    disabled?: boolean;
    label?: string;
}

const defaultAddressErrors: AddressErrors = {
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
    errors = defaultAddressErrors,
    disabled = false,
    label = "Cím"
}: AddressProps) {
    return (
        <>
            <form
                className="space-y-4"
                autoComplete={"off"}
            >
                <FieldSet>
                    <FieldLegend>
                        {label}
                    </FieldLegend>
                    <FieldGroup>
                        <Field>
                            <FieldLabel htmlFor="type">
                                Típus
                            </FieldLabel>
                            <Select>
                                <SelectTrigger className={"w-full"}>
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
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="country_code">
                                Ország kód
                            </FieldLabel>
                            <Input
                                id="country_code"
                                type="text"
                                placeholder="HU"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="postal_code">
                                Irányítószám
                            </FieldLabel>
                            <Input
                                id="postal_code"
                                type="text"
                                placeholder="1132"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="settlement">
                                Település
                            </FieldLabel>
                            <Input
                                id="settlement"
                                type="text"
                                placeholder="Budapest"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="mailbox">
                                Postafiók
                            </FieldLabel>
                            <Input
                                id="mailbox"
                                type="text"
                                placeholder="123"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="topographic_number">
                                Helyrajzi szám
                            </FieldLabel>
                            <Input
                                id="topographic_number"
                                type="text"
                                placeholder="123/A"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="name_of_public_space">
                                Közterület neve
                            </FieldLabel>
                            <Input
                                id="name_of_public_space"
                                type="text"
                                placeholder="Váci"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="type_of_public_space">
                                Közterület neve
                            </FieldLabel>
                            <Input
                                id="type_of_public_space"
                                type="text"
                                placeholder="út"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="house_number">
                                Házszám
                            </FieldLabel>
                            <Input
                                id="house_number"
                                type="text"
                                placeholder="1234"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="building">
                                Épület
                            </FieldLabel>
                            <Input
                                id="building"
                                type="text"
                                placeholder="A"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="stairway">
                                Lépcsőház
                            </FieldLabel>
                            <Input
                                id="stairway"
                                type="text"
                                placeholder="1"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="floor">
                                Emelet
                            </FieldLabel>
                            <Input
                                id="floor"
                                type="text"
                                placeholder="2"
                            />
                        </Field>

                        <Field>
                            <FieldLabel htmlFor="door">
                                Ajtó
                            </FieldLabel>
                            <Input
                                id="door"
                                type="text"
                                placeholder="3"
                            />
                        </Field>


                    </FieldGroup>
                </FieldSet>
            </form>
        </>
    )
};
