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

import { type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";
import { Slot } from "radix-ui";

import { Separator } from "@/components/ui/separator";
import type { buttonGroupVariants } from "@/components/ui/button-group-lib";

function ButtonGroup({
    className,
    orientation,
    ...props
}: React.ComponentProps<"div"> & VariantProps<typeof buttonGroupVariants>) {
    return (
        <div
            role="group"
            data-slot="button-group"
            data-orientation={orientation}
            className={cn(buttonGroupVariants({ orientation }), className)}
            {...props}
        />
    );
}

function ButtonGroupText({
    className,
    asChild = false,
    ...props
}: React.ComponentProps<"div"> & {
    asChild?: boolean;
}) {
    const Comp = asChild ? Slot.Root : "div";

    return (
        <Comp
            className={cn(
                "flex items-center gap-2 rounded-md border bg-muted px-4 text-sm font-medium shadow-xs [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
                className,
            )}
            {...props}
        />
    );
}

function ButtonGroupSeparator({
    className,
    orientation = "vertical",
    ...props
}: React.ComponentProps<typeof Separator>) {
    return (
        <Separator
            data-slot="button-group-separator"
            orientation={orientation}
            className={cn(
                "relative m-0! self-stretch bg-input data-[orientation=vertical]:h-auto",
                className,
            )}
            {...props}
        />
    );
}

export { ButtonGroup, ButtonGroupSeparator, ButtonGroupText };
