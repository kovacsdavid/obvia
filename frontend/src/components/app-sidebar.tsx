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
  SidebarRail,
} from "@/components/ui/sidebar"
import { useAuth } from "@/context/AuthContext";
import {logoutUser} from "@/store/slices/auth.ts";
import {useAppDispatch} from "@/store/hooks.ts";
import {Link, useLocation} from "react-router-dom";
import {Button} from "@/components/ui";
import {KeyRound, LogOut, NotebookPen} from "lucide-react";

export function AppSidebar({...props}: React.ComponentProps<typeof Sidebar>) {
  const dispatch = useAppDispatch();
  const location = useLocation();
  const {isLoggedIn} = useAuth();
  const handleLogout = () => {
    dispatch(logoutUser());
  }
  const data = {
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
            isActive: location.pathname === "/bejelentkezes",
          },
          {
            title: "Regisztráció",
            url: "/regisztracio",
            icon: <NotebookPen />,
            publicOnly: true,
            isActive: location.pathname === "/regisztracio",
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
    ],
  }
  return (
      <Sidebar {...props}>
        <SidebarHeader>
          {isLoggedIn ? <UserData /> : null}
        </SidebarHeader>
        <SidebarContent>
          {data.navMain.map((item) => (
              item.items.length === 0 ? null :
                  <SidebarGroup key={item.title}>
                    <SidebarGroupLabel>{item.title}</SidebarGroupLabel>
                    <SidebarGroupContent>
                      <SidebarMenu>
                        {item.items.filter((item) => (
                          !((item.private && !isLoggedIn) || item.publicOnly && isLoggedIn)
                          )).map((item) => (
                            <SidebarMenuItem key={item.title}>
                              <SidebarMenuButton asChild isActive={item.isActive}>
                                {
                                  item.url
                                      ? <Link to={item.url} key={item.title}>
                                        {item.title}
                                        { item.icon ? item.icon : "" }
                                      </Link>
                                      : item.click
                                        ? <Button
                                              key={item.title}
                                              onClick={item.click}
                                              asChild
                                              variant={"ghost"}
                                          >
                                            <Link to={item.url!} className="justify-start">
                                              {item.title}
                                              { item.icon ? item.icon : "" }
                                            </Link>
                                          </Button> :  (
                                              <span key={item.title}>{item.title}</span>
                                          )}
                              </SidebarMenuButton>
                            </SidebarMenuItem>
                        ))}
                      </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
          ))}
        </SidebarContent>
        <SidebarRail/>
      </Sidebar>
  )
}
