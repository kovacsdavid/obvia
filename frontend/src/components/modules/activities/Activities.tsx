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

import React from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@radix-ui/react-label";
import { Separator } from "@/components/ui/separator";
import {
  Item,
  ItemContent,
  ItemDescriptionLong,
  ItemTitle,
} from "@/components/ui/item";
import { Button } from "@/components/ui";

interface ActivityProps {
  resourceId: string;
  resourceType: string;
}

export default function Activities({
  resourceId,
  resourceType,
}: ActivityProps) {
  const handleSubmit = (e: React.SubmitEvent) => {
    e.preventDefault();
    console.log(resourceId, resourceType);
  };

  return (
    <Card className={"max-w-5xl mx-auto mt-5"}>
      <CardHeader>
        <CardTitle>Tevékenység</CardTitle>
      </CardHeader>
      <CardContent>
        <Item variant="outline" className="mb-3">
          <ItemContent>
            <ItemTitle>
              Kovács Dávid &lt;kapcsolat@kovacsdavid.dev&gt;{" "}
              <span className="font-normal text-xs">2026. 03. 07. 10.32</span>
            </ItemTitle>
            <ItemDescriptionLong>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
              enim ad minim veniam, quis nostrud exercitation ullamco laboris
              nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
              reprehenderit in voluptate velit esse cillum dolore eu fugiat
              nulla pariatur. Excepteur sint occaecat cupidatat non proident,
              sunt in culpa qui officia deserunt mollit anim id est laborum.
            </ItemDescriptionLong>
          </ItemContent>
        </Item>
        <Item variant="outline" className="mb-3">
          <ItemContent>
            <ItemTitle>
              Kovács Dávid &lt;kapcsolat@kovacsdavid.dev&gt;{" "}
              <span className="font-normal text-xs">2026. 03. 07. 11.32</span>
            </ItemTitle>
            <ItemDescriptionLong>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
              enim ad minim veniam, quis nostrud exercitation ullamco laboris
              nisi ut aliquip ex ea commodo consequat.
            </ItemDescriptionLong>
          </ItemContent>
        </Item>
        <Separator className="mb-3 mt-5" />
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <Label htmlFor="comment">Megjegyzés</Label>
          <Textarea id="comment" />
          <Button type="submit">Küldés</Button>
        </form>
      </CardContent>
    </Card>
  );
}
