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

import { Link, useLocation } from "react-router-dom";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { useAuth } from "@/context/AuthContext";
import {logoutUser} from "@/store/slices/auth.ts";
import {useAppDispatch, useAppSelector} from "@/store/hooks.ts";
import { LogOut, KeyRound, NotebookPen } from "lucide-react";
import type {RootState} from "@/store";

type NavItem = {
  label: string;
  to?: string;
  click?: () => void;
  icon?: React.ReactNode;
  subItems?: { label: string; to: string }[];
  private?: boolean;
  publicOnly?: boolean;
};

export function UserData() {
  const user = useAppSelector(
    (state: RootState) => state.auth.login.user
  );
  if (
    typeof user?.first_name === "string"
    && typeof user?.last_name === "string"
    && typeof user?.email === "string"
  ) {
    return (
      <div className="flex gap-2" style={{ color: "gray" }}>
        <div>
          <span className="text-sm m-0 p-0">
            { user?.last_name ? user.last_name + " " : "" }
            { user?.first_name ? user.first_name : "" } <br/>
          </span>
          <span className="text-xs block -mt-1 p-0">
            { user?.email ? "<" + user.email + ">" : "" }
          </span>
        </div>
      </div>
    )
  } else {
    return (
      <div className="flex gap-2" style={{ color: "gray" }}></div>
    )
  }
}

export function Navigation() {
  const dispatch = useAppDispatch();
  const location = useLocation();
  const { isLoggedIn } = useAuth();

  const handleLogout = () => {
    dispatch(logoutUser());
  }

  const navItems: NavItem[] = [
    {
      label: "Bejelentkezés",
      to: "/bejelentkezes",
      icon: <KeyRound />,
      publicOnly: true,
    },
    {
      label: "Kijelentkezés",
      click: handleLogout,
      icon: <LogOut />,
      private: true,

    },
    {
      label: "Regisztráció",
      to: "/regisztracio",
      icon: <NotebookPen />,
      publicOnly: true,
    },
  ];

  return (
    <nav className="border-b px-4 py-3 flex gap-4 items-center">
      <span className="font-bold text-lg">obvia://</span>
      {isLoggedIn && <UserData/>}
      <div className="flex gap-2 ml-auto">
        {navItems
          .filter((item) => {
            return !((item.private && !isLoggedIn) || item.publicOnly && isLoggedIn);
          })
          .map((item) =>
            item.subItems ? (
              <DropdownMenu key={item.label}>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost">{item.label}</Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                  {item.subItems.map((sub) => (
                    <DropdownMenuItem asChild key={sub.to}>
                      <Link
                        to={sub.to}
                        className={
                          location.pathname === sub.to ? "font-bold" : ""
                        }
                      >
                        {sub.label}
                      </Link>
                    </DropdownMenuItem>
                  ))}
                </DropdownMenuContent>
              </DropdownMenu>
            ) : item.to ? (
              <Button
                key={item.to}
                asChild
                variant={"ghost"}
              >
                <Link to={item.to!}>
                  {item.label}
                  { item.icon ? item.icon : "" }
                </Link>
              </Button>
            ) : item.click ? (
              <Button
                key={item.to}
                onClick={item.click}
                asChild
                variant={"ghost"}
              >
                <Link to={item.to!}>
                  {item.label}
                  { item.icon ? item.icon : "" }
                </Link>
              </Button>
            ) : (
              <span key={item.label}>{item.label}</span>
            )
          )}
      </div>

    </nav>
  );
}
