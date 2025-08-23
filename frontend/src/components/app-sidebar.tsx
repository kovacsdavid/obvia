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

import * as React from "react"

import { UserData } from "@/components/user-data.tsx"
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail, useSidebar,
} from "@/components/ui/sidebar"
import { useAuth } from "@/context/AuthContext";
import {logoutUser} from "@/store/slices/auth.ts";
import {useAppDispatch} from "@/store/hooks.ts";
import {Link, useLocation} from "react-router-dom";
import {Button} from "@/components/ui";
import {KeyRound, LogOut, NotebookPen, Boxes, ScrollText} from "lucide-react";

interface NavigationItem {
  title: string;
  url?: string;
  click?: () => void;
  publicOnly?: boolean;
  private?: boolean;
  icon?: React.ReactNode;
  isActive: boolean;
}

interface NavigationSection {
  title: string;
  url: string;
  items: NavigationItem[];
}

interface NavigationData {
  navMain: NavigationSection[];
}
export function AppSidebar({...props}: React.ComponentProps<typeof Sidebar>) {
  const dispatch = useAppDispatch();
  const location = useLocation();
  const {isLoggedIn} = useAuth();
  const handleLogout = () => {
    dispatch(logoutUser());
  }
  const { toggleSidebar, isMobile } = useSidebar();

  const mobileCloseOnClick = () => {
      if (isMobile) {
        toggleSidebar();
      }
  };
  const data: NavigationData = {
    navMain: [
      {
        title: "Fiók",
        url: "#",
        items: [
          {
            title: "Bejelentkezes",
            url: "/bejelentkezes",
            publicOnly: true,
            icon: <KeyRound />,
            isActive: location.pathname.includes("/bejelentkezes"),
          },
          {
            title: "Regisztráció",
            url: "/regisztracio",
            icon: <NotebookPen />,
            publicOnly: true,
            isActive: location.pathname.includes("/regisztracio"),
          },
          {
            title: "Kijelentkezés",
            click: handleLogout,
            icon: <LogOut />,
            private: true,
            isActive: false,
          },
        ],
      },
      {
        title: "Szervezeti egység",
        url: "#",
        items: [
          {
            title: "Létrehozás",
            url: "/szervezeti_egyseg/letrehozas",
            private: true,
            icon: <Boxes />,
            isActive: location.pathname.includes("/szervezeti_egyseg/letrehozas"),
          },
          {
            title: "Lista",
            url: "/szervezeti_egyseg/lista",
            private: true,
            icon: <ScrollText />,
            isActive: location.pathname.includes("/szervezeti_egyseg/lista"),
          }
        ]
      }
    ],
  }
  return (
      <Sidebar {...props}>
        <SidebarHeader>
          {isLoggedIn ? <UserData /> : null}
        </SidebarHeader>
        <SidebarContent>
          {data.navMain.map((item) => {
            const filteredItems = item.items.filter((item) => (
              !((item.private && !isLoggedIn) || item.publicOnly && isLoggedIn)
            ));
            return (
              filteredItems.length === 0 ? null :
                  <SidebarGroup key={item.title}>
                    <SidebarGroupLabel>{item.title}</SidebarGroupLabel>
                    <SidebarGroupContent>
                      <SidebarMenu>
                        {filteredItems.map((item) => (
                            <SidebarMenuItem key={item.title}>
                              <SidebarMenuButton asChild isActive={item.isActive}>
                                {
                                  item.url
                                      ? <Link onClick={mobileCloseOnClick} to={item.url} key={item.title}>
                                        {item.title}
                                        { item.icon ? item.icon : "" }
                                      </Link>
                                      : typeof item.click === "function"
                                        ? <Button
                                              key={item.title}
                                              onClick={() => {
                                                mobileCloseOnClick();
                                                // TODO: figure out why this if needed for ts
                                                if (typeof item.click === "function") {
                                                  item.click();
                                                }
                                              }}
                                              asChild
                                              variant={"ghost"}
                                          >
                                            <div className="justify-start cursor-pointer">
                                              {item.title}
                                              { item.icon ? item.icon : "" }
                                            </div>
                                          </Button> :  (
                                              <span key={item.title}>{item.title}</span>
                                          )}
                              </SidebarMenuButton>
                            </SidebarMenuItem>
                        ))}
                      </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
          )})}
        </SidebarContent>
        <SidebarRail/>
      </Sidebar>
  )
}
