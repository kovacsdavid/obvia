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

import {activate} from "@/components/tenants/slice.ts";
import {useAppDispatch} from "@/store/hooks.ts";
import {updateToken} from "@/components/auth/slice.ts";

export function useActivateTenant() {
  const dispatch = useAppDispatch();
  return async (new_tenant_id: string): Promise<boolean> => {
    return dispatch(activate(new_tenant_id)).then(async (response) => {
      if(activate.fulfilled.match(response)
        && response.payload.statusCode === 200) {
        dispatch(updateToken(response.payload.jsonData.data));
        return true;
      }
      return false;
    })
  }
}