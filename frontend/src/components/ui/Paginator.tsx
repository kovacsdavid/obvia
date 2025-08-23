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

import React from 'react';
import {
  Pagination,
  PaginationContent, PaginationEllipsis, PaginationFirst,
  PaginationItem, PaginationLast,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination"

interface PaginatorProps {
  page: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}

export const Paginator: React.FC<PaginatorProps> = ({ page, totalPages, onPageChange }) => {
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
};

export default Paginator;