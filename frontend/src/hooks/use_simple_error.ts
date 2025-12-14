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

import type { SimpleError } from "@/lib/interfaces/common";
import { useCallback, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@/context/AuthContext.tsx";

export function useSimpleError() {
  const [errors, setErrors] = useState<SimpleError | null>(null);
  const navigate = useNavigate();
  const { logout } = useAuth();

  const unexpectedError = useCallback(
    (statusCode: number | null = null) => {
      switch (statusCode) {
        case 401:
          logout();
          navigate("/bejelentkezes");
          break;
        case 429:
          setErrors({
            message:
              "Túl sok kérés érkezett rövid időn belül, ezért a szerver ideiglenesen korlátozta a hozzáférést. Próbáld újra néhány másodperc múlva!",
          });
          break;
        default:
          setErrors({
            message: "Váratlan hiba történt a feldolgozás során!",
          });
      }
    },
    [logout, navigate],
  );

  return {
    errors,
    setErrors,
    unexpectedError,
  };
}
