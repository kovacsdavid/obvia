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
    ItemActions,
    ItemContent,
    ItemDescriptionLong,
    ItemMedia,
    ItemTitle,
} from "@/components/ui/item";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu.tsx";
import { Button, FieldErrorV2 } from "@/components/ui";
import { MessageCircle, Newspaper, MoreHorizontal, Trash } from "lucide-react";
import {
    type ActivityFeedResolvedEntry,
    type CommentError,
} from "@/components/modules/activity_feed/lib/interface";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    postComment,
    list,
    deleteComment,
} from "@/components/modules/activity_feed/lib/slice.ts";
import { useDataDisplayCommon } from "@/hooks/use_data_display_common.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { formatDateLocal } from "@/lib/utils.ts";
import { useAuth } from "@/hooks/use_auth";

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
    const [errors, setErrors] = React.useState<CommentError | null>(null);
    const dispatch = useAppDispatch();
    const { claims } = useAuth();

    const { setPage, setLimit, setTotal } = useDataDisplayCommon(null);

    const refresh = useCallback(() => {
        const request = dispatch(list({ resourceId, resourceType }));
        request.then((response) => {
            if (list.fulfilled.match(response)) {
                handleApiResult(response.payload, {
                    onSuccess: ({ data, meta }) => {
                        if (data !== undefined && meta !== undefined) {
                            setPage(meta.page);
                            setLimit(meta.limit);
                            setTotal(meta.total);
                            setActivityFeed(data);
                        }
                    },
                });
            }
        });
        return request;
    }, [dispatch, resourceId, resourceType, setPage, setLimit, setTotal]);

    useEffect(() => {
        const request = refresh();
        return () => request.abort();
    }, [refresh]);

    const handleSubmit = (e: React.SubmitEvent) => {
        e.preventDefault();
        setErrors(null);
        dispatch(
            postComment({ resourceId, resourceType, comment: newComment }),
        ).then((response) => {
            if (postComment.fulfilled.match(response)) {
                handleApiResult(response.payload, {
                    onSuccess: () => {
                        refresh();
                        setNewComment("");
                    },
                    onFieldErrors: setErrors,
                });
            }
        });
    };

    const handleDelete = (id: string) => {
        dispatch(deleteComment({ uuid: id })).then((response) => {
            if (deleteComment.fulfilled.match(response)) {
                handleApiResult(response.payload, {
                    onSuccess: () => {
                        refresh();
                    },
                });
            }
        });
    };

    return (
        <>
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
                                        <Item
                                            key={item.id}
                                            variant="outline"
                                            className="mb-3"
                                        >
                                            <ItemMedia>
                                                <MessageCircle className="size-5" />
                                            </ItemMedia>
                                            <ItemContent>
                                                <ItemTitle>
                                                    {item.created_by}
                                                </ItemTitle>
                                                <ItemDescriptionLong className="mb-3">
                                                    {formatDateLocal(
                                                        item.created_at,
                                                        claims?.loc,
                                                        claims?.tz,
                                                    )}
                                                </ItemDescriptionLong>
                                                <ItemDescriptionLong>
                                                    {item.content}
                                                </ItemDescriptionLong>
                                            </ItemContent>
                                            <ItemActions>
                                                <DropdownMenu>
                                                    <DropdownMenuTrigger
                                                        asChild
                                                    >
                                                        <Button
                                                            variant="ghost"
                                                            className="h-8 w-8 p-0"
                                                        >
                                                            <span className="sr-only">
                                                                Menü megnyitása
                                                            </span>
                                                            <MoreHorizontal />
                                                        </Button>
                                                    </DropdownMenuTrigger>
                                                    <DropdownMenuContent
                                                        side={"bottom"}
                                                        align="start"
                                                    >
                                                        <DropdownMenuLabel>
                                                            Műveletek
                                                        </DropdownMenuLabel>
                                                        <DropdownMenuItem
                                                            className={
                                                                "cursor-pointer"
                                                            }
                                                            onClick={() =>
                                                                handleDelete(
                                                                    item.id,
                                                                )
                                                            }
                                                        >
                                                            <Trash /> Törlés
                                                        </DropdownMenuItem>
                                                    </DropdownMenuContent>
                                                </DropdownMenu>
                                            </ItemActions>
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
                                                <ItemTitle>
                                                    {item.content}
                                                </ItemTitle>
                                                <ItemDescriptionLong>
                                                    {formatDateLocal(
                                                        item.created_at,
                                                        claims?.loc,
                                                        claims?.tz,
                                                    )}
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
                            aria-invalid={typeof errors?.comment === "string"}
                            aria-describedby={
                                errors?.comment ? "comment-error" : undefined
                            }
                        />
                        <div id="comment-error">
                            <FieldErrorV2 error={errors?.comment} />
                        </div>
                        <FieldErrorV2 error={errors?.id} />
                        <FieldErrorV2 error={errors?.commentable_id} />
                        <FieldErrorV2 error={errors?.commentable_type} />
                        <Button type="submit">Küldés</Button>
                    </form>
                </CardContent>
            </Card>
        </>
    );
}
