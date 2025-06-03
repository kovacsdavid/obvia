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

type NavItem = {
  label: string;
  to?: string;
  subItems?: { label: string; to: string }[];
  private?: boolean;
  publicOnly?: boolean;
};

const navItems: NavItem[] = [
  /*{
    label: "About",
    to: "/about",
    publicOnly: true,
    subItems: [
      { label: "Team", to: "/about/team" },
      { label: "Company", to: "/about/company" },
    ],
  },
  {
    label: "Dashboard",
    to: "/dashboard",
    private: true,
    subItems: [
      { label: "Profile", to: "/dashboard/profile" },
      { label: "Settings", to: "/dashboard/settings" },
    ],
  },*/
  {
    label: "Bejelentkezés",
    to: "/bejelentkezes",
    publicOnly: true,
  },
  {
    label: "Kijelentkezés",
    to: "/kijelentkezes",
    private: true,
  },
  {
    label: "Regisztráció",
    to: "/regisztracio",
    publicOnly: true,
  },

];

export function Navigation() {
  const location = useLocation();
  const { isLoggedIn } = useAuth();

  return (
    <nav className="border-b px-4 py-3 flex gap-4 items-center">
      <span className="font-bold text-lg">obvia://</span>
      <div className="flex gap-2">
        {navItems
          .filter((item) => {
            if (item.private && !isLoggedIn) return false;
            if (item.publicOnly && isLoggedIn) return false;
            return true;
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
            ) : (
              <Button
                key={item.to}
                asChild
                variant={"ghost"}
              >
                <Link to={item.to!}>{item.label}</Link>
              </Button>
            )
          )}
      </div>
    </nav>
  );
}
