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
  ItemMedia,
  ItemTitle,
} from "@/components/ui/item";
import { Button } from "@/components/ui";
import { MessageCircle, Newspaper } from "lucide-react";
import { type ActivityFeedEntry } from "@/components/modules/activity_feed/lib/interface";

interface ActivityProps {
  resourceId: string;
  resourceType: string;
}

export default function ActivityFeed({
  resourceId,
  resourceType,
}: ActivityProps) {
  const [activityFeed] = React.useState<ActivityFeedEntry[]>([
    {
      activity_type: "comment",
      description: `
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
              enim ad minim veniam, quis nostrud exercitation ullamco laboris
              nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
              reprehenderit in voluptate velit esse cillum dolore eu fugiat
              nulla pariatur. Excepteur sint occaecat cupidatat non proident,
              sunt in culpa qui officia deserunt mollit anim id est laborum.
      `,
      created_at: "2026. 03. 07. 10.32",
      created_by: "Kovács Dávid <kapcsolat@kovacsdavid.dev>",
    },
    {
      activity_type: "activity",
      description: `
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua.
      `,
      created_at: "2026. 03. 07. 10.32",
      created_by: "Kovács Dávid <kapcsolat@kovacsdavid.dev>",
    },
    {
      activity_type: "activity",
      description: `
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua.
      `,
      created_at: "2026. 03. 07. 10.32",
      created_by: "Kovács Dávid <kapcsolat@kovacsdavid.dev>",
    },
    {
      activity_type: "comment",
      description: `
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
              enim ad minim veniam, quis nostrud exercitation ullamco laboris
              nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
              reprehenderit in voluptate velit esse cillum dolore eu fugiat
              nulla pariatur. Excepteur sint occaecat cupidatat non proident,
              sunt in culpa qui officia deserunt mollit anim id est laborum.
      `,
      created_at: "2026. 03. 07. 10.32",
      created_by: "Kovács Dávid <kapcsolat@kovacsdavid.dev>",
    },
  ]);

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
        {activityFeed.length > 0 &&
          activityFeed.map((item) => {
            switch (item.activity_type) {
              case "comment":
                return (
                  <Item variant="outline" className="mb-3">
                    <ItemMedia>
                      <MessageCircle className="size-5" />
                    </ItemMedia>
                    <ItemContent>
                      <ItemTitle>{item.created_by} </ItemTitle>
                      <ItemDescriptionLong>
                        {item.created_at}
                      </ItemDescriptionLong>
                      <ItemDescriptionLong>
                        {item.description}
                      </ItemDescriptionLong>
                    </ItemContent>
                  </Item>
                );
              case "activity":
                return (
                  <Item variant="outline" size="sm" className="mb-3">
                    <ItemMedia>
                      <Newspaper className="size-5" />
                    </ItemMedia>
                    <ItemContent>
                      <ItemTitle>{item.description}</ItemTitle>
                      <ItemDescriptionLong>
                        {item.created_at}
                      </ItemDescriptionLong>
                    </ItemContent>
                  </Item>
                );
              default:
                return null;
            }
          })}
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
