/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import React, { useCallback, useEffect } from "react";
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
import { Button, GlobalError } from "@/components/ui";
import { MessageCircle, Newspaper } from "lucide-react";
import { type ActivityFeedResolvedEntry } from "@/components/modules/activity_feed/lib/interface";
import { useAppDispatch } from "@/store/hooks.ts";
import {
  postComment,
  list,
} from "@/components/modules/activity_feed/lib/slice.ts";
import { useDataDisplayCommon } from "@/hooks/use_data_display_common.ts";
import { useSimpleError } from "@/hooks/use_simple_error.ts";
import { formatDateToYMDHMS } from "@/lib/utils.ts";

interface ActivityProps {
  resourceId: string;
  resourceType: string;
}

export default function ActivityFeed({
  resourceId,
  resourceType,
}: ActivityProps) {
  const [activityFeed, setActivityFeed] = React.useState<
    ActivityFeedResolvedEntry[]
  >([]);
  const [newComment, setNewComment] = React.useState("");
  const dispatch = useAppDispatch();
  const { errors, setErrors, unexpectedError } = useSimpleError();

  const { setPage, setLimit, setTotal } = useDataDisplayCommon(null);

  const refresh = useCallback(() => {
    dispatch(list({ resourceId, resourceType })).then(async (response) => {
      if (list.fulfilled.match(response)) {
        if (
          response.payload.statusCode === 200 &&
          typeof response.payload.jsonData?.data !== "undefined" &&
          typeof response.payload.jsonData?.meta !== "undefined"
        ) {
          setPage(response.payload.jsonData.meta.page);
          setLimit(response.payload.jsonData.meta.limit);
          setTotal(response.payload.jsonData.meta.total);
          setActivityFeed(response.payload.jsonData.data);
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    dispatch,
    resourceId,
    resourceType,
    setPage,
    setLimit,
    setTotal,
    setErrors,
    unexpectedError,
  ]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleSubmit = (e: React.SubmitEvent) => {
    e.preventDefault();
    dispatch(
      postComment({ resourceId, resourceType, comment: newComment }),
    ).then(async (response) => {
      if (postComment.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          refresh();
          setNewComment("");
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors} />
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
                    <Item key={item.id} variant="outline" className="mb-3">
                      <ItemMedia>
                        <MessageCircle className="size-5" />
                      </ItemMedia>
                      <ItemContent>
                        <ItemTitle>{item.created_by}</ItemTitle>
                        <ItemDescriptionLong className="mb-3">
                          {formatDateToYMDHMS(item.created_at)}
                        </ItemDescriptionLong>
                        <ItemDescriptionLong>
                          {item.content}
                        </ItemDescriptionLong>
                      </ItemContent>
                    </Item>
                  );
                case "activity":
                  return (
                    <Item
                      key={item.id}
                      variant="outline"
                      size="sm"
                      className="mb-3"
                    >
                      <ItemMedia>
                        <Newspaper className="size-5" />
                      </ItemMedia>
                      <ItemContent>
                        <ItemTitle>{item.content}</ItemTitle>
                        <ItemDescriptionLong>
                          {formatDateToYMDHMS(item.created_at)}
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
            <Textarea
              id="comment"
              value={newComment}
              onChange={(e) => setNewComment(e.target.value)}
            />
            <Button type="submit">Küldés</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}
