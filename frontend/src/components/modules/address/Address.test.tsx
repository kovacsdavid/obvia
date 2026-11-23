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
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { Address as AddressValue } from "./lib/interface";
import Address from "./Address";

const mocks = vi.hoisted(() => ({ dispatch: vi.fn(), toast: vi.fn() }));
vi.mock("@/store/hooks", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/components/modules/address/lib/slice", () => ({
    select_list: (args: unknown, options: unknown) => ({ args, options }),
}));
vi.mock("@/components/ui/toast-lib", () => ({ toast: { add: mocks.toast } }));
vi.mock("@/components/ui/combobox", () => ({
    Combobox: ({
        children,
        value,
        onValueChange,
    }: {
        children: React.ReactNode;
        value: string;
        onValueChange: (value: string) => void;
    }) => (
        <div>
            <button type="button" onClick={() => onValueChange("US")}>
                Select US
            </button>
            <span>{value}</span>
            {children}
        </div>
    ),
    ComboboxInput: (props: React.ComponentProps<"input">) => (
        <input {...props} />
    ),
    ComboboxContent: () => null,
    ComboboxEmpty: () => null,
    ComboboxItem: () => null,
    ComboboxList: () => null,
}));

const address: AddressValue = {
    id: "address-1",
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
const success = {
    kind: "success",
    status: 200,
    data: {
        data: [
            { value: "HU", title: "Hungary" },
            { value: "US", title: "United States" },
        ],
    },
};

describe("Address", () => {
    beforeEach(() => {
        mocks.dispatch.mockReset();
        mocks.toast.mockReset();
        mocks.dispatch.mockReturnValue({
            unwrap: () => Promise.resolve(success),
        });
    });
    afterEach(cleanup);

    it("loads countries without changing the address and preserves field edits", async () => {
        const onChange = vi.fn();
        const setErrors = vi.fn();
        render(
            <Address
                value={address}
                onChange={onChange}
                setErrors={setErrors}
            />,
        );
        expect(
            screen.queryByRole("button", { name: "Select US" }),
        ).not.toBeInTheDocument();
        await screen.findByRole("button", { name: "Select US" });
        expect(onChange).not.toHaveBeenCalled();
        expect(mocks.dispatch).toHaveBeenCalledWith(
            expect.objectContaining({
                args: { list: "countries" },
                options: { signal: expect.any(AbortSignal) },
            }),
        );
        fireEvent.change(screen.getByLabelText("Irányítószám"), {
            target: { value: "1234" },
        });
        expect(onChange).toHaveBeenLastCalledWith({
            ...address,
            postal_code: "1234",
        });
        expect(setErrors).toHaveBeenCalledWith(null, "postal_code");
        fireEvent.click(screen.getByRole("button", { name: "Select US" }));
        expect(onChange).toHaveBeenLastCalledWith({
            ...address,
            country_code: "US",
        });
        expect(setErrors).toHaveBeenCalledWith(null, "country_code");
    });

    it("shows a retry for an API failure without a parent error setter", async () => {
        mocks.dispatch.mockReturnValueOnce({
            unwrap: () =>
                Promise.resolve({
                    kind: "http_error",
                    status: 500,
                    body: { error: { message: "Load failed" } },
                }),
        });
        render(<Address value={address} onChange={vi.fn()} />);
        expect(await screen.findByText("Load failed")).toBeInTheDocument();
        fireEvent.click(
            screen.getByRole("button", { name: "Lista betöltése újra!" }),
        );
        await screen.findByRole("button", { name: "Select US" });
        expect(screen.queryByText("Load failed")).not.toBeInTheDocument();
    });

    it("handles rejected requests and aborts in-flight loads on unmount", async () => {
        mocks.dispatch.mockReturnValueOnce({
            unwrap: () => Promise.reject(new Error("failed")),
        });
        const { unmount } = render(
            <Address value={address} onChange={vi.fn()} />,
        );
        await screen.findByText("Nem sikerült betölteni az országok listáját.");
        expect(mocks.toast).toHaveBeenCalledOnce();
        const signal = mocks.dispatch.mock.calls[0][0].options
            .signal as AbortSignal;
        unmount();
        expect(signal.aborted).toBe(true);
    });

    it("retains conditional address fields and disabled input behavior", async () => {
        const { rerender } = render(
            <Address
                value={{ ...address, type: "mailbox" }}
                onChange={vi.fn()}
                disabled
            />,
        );
        expect(screen.getByLabelText("Postafiók")).toBeDisabled();
        rerender(
            <Address
                value={{ ...address, type: "topographic_number" }}
                onChange={vi.fn()}
            />,
        );
        expect(screen.getByLabelText("Helyrajzi szám")).toBeInTheDocument();
        expect(screen.queryByLabelText("Postafiók")).not.toBeInTheDocument();
        await waitFor(() => expect(mocks.dispatch).toHaveBeenCalled());
    });

    it("does not reload when the parent replaces its error callback", async () => {
        let resolveList: (result: unknown) => void = () => {};
        mocks.dispatch.mockReturnValueOnce({
            unwrap: () =>
                new Promise((resolve) => {
                    resolveList = resolve;
                }),
        });
        const firstSetter = vi.fn();
        const latestSetter = vi.fn();
        const { rerender } = render(
            <Address
                value={address}
                onChange={vi.fn()}
                setErrors={firstSetter}
            />,
        );
        rerender(
            <Address
                value={address}
                onChange={vi.fn()}
                setErrors={latestSetter}
            />,
        );
        resolveList({
            kind: "success",
            status: 200,
            data: { error: { message: "Country list error" } },
        });
        await waitFor(() =>
            expect(latestSetter).toHaveBeenCalledWith(
                "Country list error",
                "country_code",
            ),
        );
        expect(firstSetter).not.toHaveBeenCalled();
        expect(mocks.dispatch).toHaveBeenCalledOnce();
    });
});
