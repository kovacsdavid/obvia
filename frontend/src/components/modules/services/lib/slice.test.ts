import { configureStore } from "@reduxjs/toolkit";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { RootState } from "@/store";
import type { ServiceUserInput } from "./interface";
import * as servicesApi from "./service";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import * as services from "./slice";

vi.mock("./service", () => ({
    create: vi.fn(),
    update: vi.fn(),
    list: vi.fn(),
    get: vi.fn(),
    get_resolved: vi.fn(),
    deleteItem: vi.fn(),
    select_list: vi.fn(),
    print: vi.fn(),
}));

vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));

vi.mock("@/components/modules/auth/lib/slice.ts", () => ({
    refreshAccessToken:
        () => async (dispatch: (action: { type: string }) => void) => {
            await Promise.resolve();
            dispatch({ type: "test/refreshed" });
        },
}));

const input: ServiceUserInput = {
    id: "service-1",
    name: "Installation",
    description: null,
    defaultPrice: null,
    defaultTaxId: null,
    currencyCode: null,
    status: "active",
    unitOfMeasureId: "unit-1",
    newUnitOfMeasure: "",
};

const result = { kind: "http_error", status: 500 } as const;

function makeStore() {
    const initialState = {
        auth: { login: { token: "old-token" } },
    } as RootState;
    return configureStore({
        reducer: (state = initialState, action) =>
            action.type === "test/refreshed"
                ? {
                      ...state,
                      auth: {
                          ...state.auth,
                          login: { ...state.auth.login, token: "fresh-token" },
                      },
                  }
                : state,
    });
}

type Dispatch = ReturnType<typeof makeStore>["dispatch"];

const cases = [
    {
        name: "create",
        api: servicesApi.create,
        argument: input,
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.create({ input, silent })),
    },
    {
        name: "update",
        api: servicesApi.update,
        argument: input,
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.update({ input, silent })),
    },
    {
        name: "list",
        api: servicesApi.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.list({ query: "page=2", silent })),
    },
    {
        name: "get",
        api: servicesApi.get,
        argument: "service-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.get({ uuid: "service-1", silent })),
    },
    {
        name: "get_resolved",
        api: servicesApi.get_resolved,
        argument: "service-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.get_resolved({ uuid: "service-1", silent })),
    },
    {
        name: "deleteItem",
        api: servicesApi.deleteItem,
        argument: "service-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.deleteItem({ uuid: "service-1", silent })),
    },
    {
        name: "select_list",
        api: servicesApi.select_list,
        argument: "currencies",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.select_list({ list: "currencies", silent })),
    },
    {
        name: "print",
        api: servicesApi.print,
        argument: "service-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(services.print({ uuid: "service-1", silent })),
    },
];

describe("service thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases) {
            vi.mocked(testCase.api).mockResolvedValue(result);
        }
    });

    it.each(cases)(
        "refreshes, forwards the token and signal, and notifies for $name",
        async ({ api, argument, run }) => {
            const store = makeStore();
            const action = await run(store.dispatch);
            expect(action.payload).toEqual(result);
            expect(api).toHaveBeenCalledOnce();
            expect(api).toHaveBeenCalledWith(
                argument,
                "fresh-token",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(result);
        },
    );

    it.each(cases)(
        "suppresses notifications for silent $name",
        async ({ api, argument, run }) => {
            const store = makeStore();
            const action = await run(store.dispatch, true);
            expect(action.payload).toEqual(result);
            expect(api).toHaveBeenCalledOnce();
            expect(api).toHaveBeenCalledWith(
                argument,
                "fresh-token",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).not.toHaveBeenCalled();
        },
    );
});
