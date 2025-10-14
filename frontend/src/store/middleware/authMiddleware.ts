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

import {type Middleware, type MiddlewareAPI} from '@reduxjs/toolkit';
import {logoutUser} from '@/components/modules/auth/lib/slice.ts';

const isSessionExpiredAction = (action: unknown) => (
  typeof action === "object"
  && action !== null
  && "payload" in action
  && typeof action.payload === "object"
  && action.payload !== null
  && "status" in action.payload
  && action.payload.status === 401
  && window.location.pathname !== '/bejelentkezes'
);

const authMiddleware: Middleware = (store: MiddlewareAPI) => (next: (action: unknown) => unknown) => (action: unknown) => {
  if (
    isSessionExpiredAction(action)
  ) {
    store.dispatch(logoutUser());
    window.location.href = '/bejelentkezes';
  } else {
    return next(action);
  }
};

export default authMiddleware;
