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

import type {
  CommonResponse,
  FormError,
  PaginatedDataResponse,
  SimpleError,
} from "@/lib/interfaces/common";

export interface ActivityFeedResolvedEntry {
  id: string;
  resource_id: string;
  resource_type: string;
  activity_type: string;
  content: string;
  created_by_id: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export type PostCommentResponse = CommonResponse<
  ActivityFeedResolvedEntry,
  FormError
>;

export type PaginatedActivityFeedResponse = PaginatedDataResponse<
  ActivityFeedResolvedEntry[],
  SimpleError
>;
