import {
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
    within,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { MemoryRouter } from "react-router";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { decode_url_encoded_get_query } from "@/lib/get_query";
import List from "./List";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    list: vi.fn(),
    deleteItem: vi.fn(),
    print: vi.fn(),
    openPopup: vi.fn(),
    updateWindowWithPdfData: vi.fn(),
    close: vi.fn(),
}));

vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/components/modules/services/lib/slice.ts", () => ({
    list: (payload: unknown) => ({ kind: "list", payload }),
    deleteItem: (payload: unknown) => ({ kind: "deleteItem", payload }),
    print: (payload: unknown) => ({ kind: "print", payload }),
}));
vi.mock("@/lib/utils.ts", async (importOriginal) => ({
    ...(await importOriginal<typeof import("@/lib/utils.ts")>()),
    openPopup: mocks.openPopup,
    updateWindowWithPdfData: mocks.updateWindowWithPdfData,
}));

const service = {
    id: "service-1",
    name: "Javítás",
    description: "Szerviz",
    default_price: "1200",
    default_tax_id: null,
    default_tax: null,
    currency_code: "HUF",
    status: "active",
    created_by: "Anna",
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
    unit_of_measure: "óra",
};

function listResult(page: number, data = [service]) {
    return {
        kind: "success",
        data: { meta: { page, limit: 25, total: 50 }, data },
    };
}

async function renderList() {
    render(
        <MemoryRouter>
            <List />
        </MemoryRouter>,
    );
    await screen.findByText("Javítás");
}

async function chooseAction(name: string) {
    fireEvent.pointerDown(
        screen.getByRole("button", { name: "Menü megnyitása" }),
        {
            button: 0,
            ctrlKey: false,
        },
    );
    fireEvent.click(await screen.findByRole("menuitem", { name }));
}

describe("services list", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.list.mockResolvedValue(listResult(1));
        mocks.deleteItem.mockResolvedValue({ kind: "success", data: {} });
        mocks.openPopup.mockReturnValue({ close: mocks.close });
        mocks.dispatch.mockImplementation(
            (action: {
                kind: "list" | "deleteItem" | "print";
                payload: unknown;
            }) => ({
                unwrap: () => mocks[action.kind](action.payload),
            }),
        );
    });

    afterEach(cleanup);

    it("loads services and renders their details and pagination metadata", async () => {
        await renderList();

        expect(mocks.list).toHaveBeenCalledExactlyOnceWith({ query: null });
        const row = screen.getByText("Javítás").closest("tr")!;
        expect(within(row).getByText("Szerviz")).toBeInTheDocument();
        expect(within(row).getByText("HUF")).toBeInTheDocument();
        expect(within(row).getByText("Aktív")).toBeInTheDocument();
        expect(within(row).getByText("Anna")).toBeInTheDocument();
        expect(screen.getByText("2")).toBeInTheDocument();
    });

    it("loads the selected page and refreshes the same query after deletion", async () => {
        mocks.list
            .mockResolvedValueOnce(listResult(1))
            .mockResolvedValueOnce(listResult(2))
            .mockResolvedValueOnce(listResult(2, []));
        await renderList();

        fireEvent.click(screen.getByText("2"));
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(2));
        const query = mocks.list.mock.calls[1][0].query;
        expect(decode_url_encoded_get_query(query).paging).toEqual({
            page: 2,
            limit: 25,
        });
        await waitFor(() =>
            expect(screen.getByText("2")).toHaveStyle({ fontWeight: "bolder" }),
        );

        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(3));
        expect(mocks.deleteItem).toHaveBeenCalledExactlyOnceWith({
            uuid: "service-1",
        });
        expect(mocks.list).toHaveBeenLastCalledWith({ query });
        await waitFor(() =>
            expect(screen.queryByText("Javítás")).not.toBeInTheDocument(),
        );
    });

    it.each([
        ["an API error", () => Promise.resolve({ kind: "network_error" })],
        ["a rejection", () => Promise.reject(new Error("list failed"))],
    ])(
        "keeps the existing list after a page request returns %s",
        async (_label, result) => {
            mocks.list
                .mockResolvedValueOnce(listResult(1))
                .mockImplementationOnce(result);
            await renderList();

            fireEvent.click(screen.getByText("2"));
            await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(2));
            expect(screen.getByText("Javítás")).toBeInTheDocument();
        },
    );

    it.each([
        ["an API error", () => Promise.resolve({ kind: "network_error" })],
        ["a rejection", () => Promise.reject(new Error("delete failed"))],
    ])("does not refresh after delete returns %s", async (_label, result) => {
        mocks.deleteItem.mockImplementation(result);
        await renderList();

        await chooseAction("Törlés");
        await waitFor(() => expect(mocks.deleteItem).toHaveBeenCalledOnce());
        expect(mocks.list).toHaveBeenCalledOnce();
        expect(screen.getByText("Javítás")).toBeInTheDocument();
    });

    it("updates the popup with a successful PDF response", async () => {
        const pdf = new Uint8Array([37, 80, 68, 70]);
        mocks.print.mockResolvedValue({ kind: "success", data: pdf });
        await renderList();

        await chooseAction("Nyomtatás");
        await waitFor(() =>
            expect(
                mocks.updateWindowWithPdfData,
            ).toHaveBeenCalledExactlyOnceWith(
                mocks.openPopup.mock.results[0].value,
                pdf,
            ),
        );
        expect(mocks.print).toHaveBeenCalledExactlyOnceWith({
            uuid: "service-1",
        });
        expect(mocks.close).not.toHaveBeenCalled();
    });

    it.each([
        ["an API error", () => Promise.resolve({ kind: "network_error" })],
        [
            "an empty PDF",
            () => Promise.resolve({ kind: "success", data: null }),
        ],
        ["a rejection", () => Promise.reject(new Error("print failed"))],
    ])("closes the popup after %s", async (_label, result) => {
        mocks.print.mockImplementation(result);
        await renderList();

        await chooseAction("Nyomtatás");
        await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
        expect(mocks.updateWindowWithPdfData).not.toHaveBeenCalled();
    });

    it("closes the popup if displaying the PDF throws", async () => {
        mocks.print.mockResolvedValue({
            kind: "success",
            data: new Uint8Array([37]),
        });
        mocks.updateWindowWithPdfData.mockImplementationOnce(() => {
            throw new Error("display failed");
        });
        await renderList();

        await chooseAction("Nyomtatás");
        await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
        expect(mocks.updateWindowWithPdfData).toHaveBeenCalledOnce();
    });

    it("does not request a PDF when the popup is blocked", async () => {
        mocks.openPopup.mockReturnValue(null);
        await renderList();

        await chooseAction("Nyomtatás");
        expect(mocks.openPopup).toHaveBeenCalledOnce();
        expect(mocks.print).not.toHaveBeenCalled();
        expect(mocks.updateWindowWithPdfData).not.toHaveBeenCalled();
    });
});
