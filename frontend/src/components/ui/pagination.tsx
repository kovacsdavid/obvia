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
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  MoreHorizontalIcon,
} from "lucide-react"

import { cn } from "@/lib/utils"
import { Button, buttonVariants } from "@/components/ui/button"

function Pagination({ className, ...props }: React.ComponentProps<"nav">) {
  return (
    <nav
      role="navigation"
      aria-label="pagination"
      data-slot="pagination"
      className={cn("mx-auto flex w-full justify-center", className)}
      {...props}
    />
  )
}

function PaginationContent({
  className,
  ...props
}: React.ComponentProps<"ul">) {
  return (
    <ul
      data-slot="pagination-content"
      className={cn("flex flex-row items-center gap-1", className)}
      {...props}
    />
  )
}

function PaginationItem({ ...props }: React.ComponentProps<"li">) {
  return <li data-slot="pagination-item" {...props} />
}

type PaginationLinkProps = {
  isActive?: boolean
} & Pick<React.ComponentProps<typeof Button>, "size"> &
  React.ComponentProps<"a">

function PaginationLink({
  className,
  isActive,
  size = "icon",
  ...props
}: PaginationLinkProps) {
  return (
    <a
      aria-current={isActive ? "page" : undefined}
      data-slot="pagination-link"
      data-active={isActive}
      className={cn(
        buttonVariants({
          variant: isActive ? "outline" : "ghost",
          size,
        }),
        className
      )}
      {...props}
    />
  )
}

function PaginationPrevious({
  className,
  ...props
}: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      aria-label="Go to previous page"
      size="default"
      className={cn("gap-1 px-2.5 sm:pl-2.5", className)}
      {...props}
    >
      <ChevronLeftIcon />
      <span className="hidden sm:block">Előző</span>
    </PaginationLink>
  )
}


function PaginationFirst({
                              className,
                              ...props
                            }: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      aria-label="Go to previous page"
      size="default"
      className={cn("gap-1 px-2.5 sm:pl-2.5", className)}
      {...props}
    >
      <ChevronLeftIcon style={{marginRight: "-12px"}}/>
      <ChevronLeftIcon />
      <span className="hidden sm:block">Első</span>
    </PaginationLink>
  )
}

function PaginationNext({
  className,
  ...props
}: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      aria-label="Go to next page"
      size="default"
      className={cn("gap-1 px-2.5 sm:pr-2.5", className)}
      {...props}
    >
      <span className="hidden sm:block">Következő</span>
      <ChevronRightIcon />
    </PaginationLink>
  )
}

function PaginationLast({
                          className,
                          ...props
                        }: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      aria-label="Go to next page"
      size="default"
      className={cn("gap-1 px-2.5 sm:pr-2.5", className)}
      {...props}
    >
      <span className="hidden sm:block">Utolsó</span>
      <ChevronRightIcon style={{marginRight: "-12px"}}/>
      <ChevronRightIcon />
    </PaginationLink>
  )
}

function PaginationEllipsis({
  className,
  ...props
}: React.ComponentProps<"span">) {
  return (
    <span
      aria-hidden
      data-slot="pagination-ellipsis"
      className={cn("flex size-9 items-center justify-center", className)}
      {...props}
    >
      <MoreHorizontalIcon className="size-4" />
      <span className="sr-only">More pages</span>
    </span>
  )
}


interface PaginatorProps {
  page: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}

function Paginator ({ page, totalPages, onPageChange }: PaginatorProps) {
  const pages = Array.from({ length: totalPages }, (_, i) => i + 1);

  return (
    <Pagination style={{marginTop: "50px"}}>
      <PaginationContent>
        {page > 1 && (
          <>
            <PaginationItem>
              <PaginationItem>
                <PaginationFirst
                  style={{cursor: 'pointer'}}
                  onClick={() => onPageChange(1)}
                />
              </PaginationItem>
            </PaginationItem>
            <PaginationItem>
              <PaginationPrevious
                style={{ cursor: 'pointer' }}
                onClick={() => onPageChange(page - 1)}
              />
            </PaginationItem>
          </>
        )}
        {page - 3 > 0 ? (
            <PaginationItem>
              <PaginationEllipsis/>
            </PaginationItem>)
          : null}
        {pages.map((pageNumber) => {
          if (pageNumber < page + 3 && pageNumber > page -3) {
            return (<PaginationItem key={pageNumber}>
                <PaginationLink
                  style={{cursor: 'pointer', ...(page === pageNumber ? {cursor: 'default', fontWeight: "bolder", backgroundColor: "lightgray"} : {})}}
                  onClick={() => onPageChange(pageNumber)}
                >
                  {pageNumber}
                </PaginationLink>
              </PaginationItem>
            )
          } else {
            return null
          }
        })}
        {page + 3 <= totalPages ? (
            <PaginationItem>
              <PaginationEllipsis/>
            </PaginationItem>)
          : null}
        {page < totalPages && (
          <>
            <PaginationItem>
              <PaginationNext
                style={{cursor: 'pointer'}}
                onClick={() => onPageChange(page + 1)}
              />
            </PaginationItem>
            <PaginationItem>
              <PaginationLast
                style={{cursor: 'pointer'}}
                onClick={() => onPageChange(totalPages)}
              />
            </PaginationItem>
          </>
        )}

      </PaginationContent>
    </Pagination>
  );
}

export {
  Pagination,
  PaginationContent,
  PaginationLink,
  PaginationItem,
  PaginationPrevious,
  PaginationFirst,
  PaginationNext,
  PaginationLast,
  PaginationEllipsis,
  Paginator
}
