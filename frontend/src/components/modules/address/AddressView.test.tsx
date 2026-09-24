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

import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { describe, it, expect } from "vitest";

import AddressView from "./AddressView";
import type { Address } from "@/components/modules/address/lib/interface";

const baseAddress: Address = {
    id: "",
    type: "full_address",
    country_code: "HU",
    postal_code: "1011",
    settlement: "Budapest",
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

describe("AddressView", () => {
    it("renders N/A when value is null", () => {
        render(<AddressView value={null} />);
        expect(screen.getByText("N/A")).toBeInTheDocument();
    });

    it("renders N/A when value is undefined", () => {
        render(<AddressView value={undefined} />);
        expect(screen.getByText("N/A")).toBeInTheDocument();
    });

    it("renders the base address fields", () => {
        render(<AddressView value={baseAddress} />);
        expect(screen.getByText("HU1011 Budapest,")).toBeInTheDocument();
    });

    it("renders mailbox when provided", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    mailbox: "123",
                }}
            />,
        );

        expect(
            screen.getByText("HU1011 Budapest, PF. 123"),
        ).toBeInTheDocument();
    });

    it("renders topographic number when provided", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    topographic_number: "123/A",
                }}
            />,
        );

        expect(
            screen.getByText("HU1011 Budapest, HRSZ.: 123/A"),
        ).toBeInTheDocument();
    });

    it("renders public space name, type, and house number", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                }}
            />,
        );

        expect(
            screen.getByText("HU1011 Budapest, Váci út 1111."),
        ).toBeInTheDocument();
    });

    it("renders building without trailing comma when it is the last part", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                    building: "A",
                }}
            />,
        );

        expect(
            screen.getByText("HU1011 Budapest, Váci út 1111. A épület"),
        ).toBeInTheDocument();
    });

    it("renders building with trailing comma when more address parts follow", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                    building: "A",
                    stairway: "B",
                }}
            />,
        );

        expect(
            screen.getByText(
                "HU1011 Budapest, Váci út 1111. A épület, B lépcsőház",
            ),
        ).toBeInTheDocument();
    });

    it("renders stairway without trailing comma when it is the last part", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                    building: "A",
                    stairway: "C",
                }}
            />,
        );

        expect(
            screen.getByText(
                "HU1011 Budapest, Váci út 1111. A épület, C lépcsőház",
            ),
        ).toBeInTheDocument();
    });

    it("renders stairway with trailing comma when more address parts follow", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                    stairway: "C",
                    door: "12",
                }}
            />,
        );

        expect(
            screen.getByText(
                "HU1011 Budapest, Váci út 1111. C lépcsőház, 12 ajtó",
            ),
        ).toBeInTheDocument();
    });

    it("renders door when provided", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                    door: "7",
                }}
            />,
        );

        expect(
            screen.getByText("HU1011 Budapest, Váci út 1111. 7 ajtó"),
        ).toBeInTheDocument();
    });

    it("renders a full address", () => {
        render(
            <AddressView
                value={{
                    ...baseAddress,
                    name_of_public_space: "Váci",
                    type_of_public_space: "út",
                    house_number: "1111",
                    building: "A",
                    stairway: "B",
                    floor: "1",
                    door: "2",
                }}
            />,
        );

        expect(
            screen.getByText(
                "HU1011 Budapest, Váci út 1111. A épület, B lépcsőház, 1 emelet, 2 ajtó",
            ),
        ).toBeInTheDocument();
    });
});
