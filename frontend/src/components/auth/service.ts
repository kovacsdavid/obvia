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

import {globalRequestTimeout} from "@/services/utils/consts.ts";
import type {LoginRequest, RegisterRequest} from "@/components/auth/interface.ts";

export async function login({email, password}: LoginRequest): Promise<Response> {
  return await fetch(`/api/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({email, password}),
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}

export async function register({
                                 firstName,
                                 lastName,
                                 email,
                                 password,
                                 passwordConfirm
                               }: RegisterRequest): Promise<Response> {
  return await fetch(`/api/auth/register`, {
    method: "POST",
    headers: {"Content-Type": "application/json"},
    body: JSON.stringify({
      first_name: firstName,
      last_name: lastName,
      email,
      password,
      password_confirm: passwordConfirm
    }),
    signal: AbortSignal.timeout(globalRequestTimeout)
  });
}
