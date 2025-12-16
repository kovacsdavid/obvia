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

import { Button } from "@/components/ui";
import { useAuth } from "@/context/AuthContext.tsx";
import {
  Boxes,
  Database,
  HandCoins,
  HandPlatter,
  ListTodo,
  LogOut,
  NotebookText,
  Package,
  UsersRound,
  Warehouse,
} from "lucide-react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { useNavigate } from "react-router-dom";
import { useAppDispatch } from "@/store/hooks.ts";
import { logoutUser } from "@/components/modules/auth/lib/slice.ts";

export default function Dashboard() {
  const { hasActiveDatabase } = useAuth();
  const dispatch = useAppDispatch();

  const handleLogout = () => {
    dispatch(logoutUser());
  };
  const navigate = useNavigate();
  return (
    <Card className={"max-w-3xl mx-auto"}>
      <CardHeader>
        <CardTitle>Vezérlőpult</CardTitle>
      </CardHeader>
      <CardContent className="text-center">
        <Button
          onClick={() => navigate("/adatbazis/lista")}
          variant={"outline"}
          className="p-8 m-5"
        >
          <Database /> Adatbázis
        </Button>
        <Button
          onClick={() => navigate("/vevo/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <UsersRound /> Vevők
        </Button>
        <Button
          onClick={() => navigate("/raktar/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <Warehouse /> Raktárak
        </Button>
        <Button
          onClick={() => navigate("/ado/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <HandCoins /> Adók
        </Button>
        <Button
          onClick={() => navigate("/termek/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <Package /> Termékek
        </Button>
        <Button
          onClick={() => navigate("/raktarkeszlet/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <Boxes /> Raktárkészlet
        </Button>
        <Button
          onClick={() => navigate("/szolgaltatas/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <HandPlatter /> Szolgáltatások
        </Button>
        <Button
          onClick={() => navigate("/feladat/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <ListTodo /> Feladatok
        </Button>
        <Button
          onClick={() => navigate("/munkalap/lista")}
          disabled={!hasActiveDatabase}
          variant={"outline"}
          className="p-8 m-5"
        >
          <NotebookText /> Munkalapok
        </Button>
        <Button onClick={handleLogout} variant={"outline"} className="p-8 m-5">
          <LogOut /> Kijelentkezés
        </Button>
      </CardContent>
    </Card>
  );
}
