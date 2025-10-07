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


import {
  type CommonResponse,
  type FormError,
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface WorksheetUserInput {
  id: string | null
  name: string
  description: string
  projectId: string
  status: string
}

export interface Worksheet {
  id: string,
  name: string,
  description: string | null,
  project_id: string,
  created_by_id: string,
  status: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export interface WorksheetResolved {
  id: string,
  name: string,
  description: string | null,
  project_id: string,
  project: string,
  created_by_id: string,
  created_by: string,
  status: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type CreateWorksheetResponse = CommonResponse<SimpleMessageData, FormError>;
export type UpdateWorksheetResponse = CommonResponse<SimpleMessageData, FormError>;
export type DeleteWorksheetResponse = CommonResponse<SimpleMessageData, SimpleError>;
export type WorksheetResponse = CommonResponse<Worksheet, SimpleError>;
export type WorksheetResolvedResponse = CommonResponse<WorksheetResolved, SimpleError>;
export type WorksheetResolvedList = WorksheetResolved[];
export type PaginatedWorksheetResolvedListResponse = PaginatedDataResponse<WorksheetResolvedList, SimpleError>;

