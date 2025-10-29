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

import {configureStore} from "@reduxjs/toolkit";
import {combineReducers} from "redux";
import authReducer from "@/components/modules/auth/lib/slice.ts";
import tenantsReducer from "@/components/modules/databases/lib/slice.ts";
import customersReducer from "@/components/modules/customers/lib/slice.ts";
import inventoryReducer from "@/components/modules/inventory/lib/slice.ts";
import productsReducer from "@/components/modules/products/lib/slice.ts";
import projectsReducer from "@/components/modules/projects/lib/slice.ts";
import tagsReducer from "@/components/modules/tags/lib/slice.ts";
import tasksReducer from "@/components/modules/tasks/lib/slice.ts";
import usersReducer from "@/components/modules/users/lib/slice.ts";
import warehousesReducer from "@/components/modules/warehouses/lib/slice.ts";
import worksheetsReducer from "@/components/modules/worksheets/lib/slice.ts";
import inventoryMovementsReducer from "@/components/modules/inventory_movements/lib/slice.ts";
import storage from "redux-persist/lib/storage";
import {persistReducer, persistStore} from "redux-persist";
import authMiddleware from "@/store/middleware/authMiddleware.ts";

const rootReducer = combineReducers({
  auth: authReducer,
  tenants: tenantsReducer,
  customers: customersReducer,
  inventory: inventoryReducer,
  inventory_movements: inventoryMovementsReducer,
  products: productsReducer,
  projects: projectsReducer,
  tags: tagsReducer,
  tasks: tasksReducer,
  users: usersReducer,
  warehouses: warehousesReducer,
  worksheets: worksheetsReducer,
})

const persistConfig = {
  key: "root",
  storage,
  whitelist: ["auth"]
}

const persistedReducer = persistReducer(persistConfig, rootReducer);

export const store = configureStore({
  reducer: persistedReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: false
    }).concat(authMiddleware)
});

export const persistor = persistStore(store);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
