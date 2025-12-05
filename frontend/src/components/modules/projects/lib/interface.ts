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
  type SimpleMessageData,
} from "@/lib/interfaces/common.ts";

export interface ProjectUserInput {
  id: string | null;
  name: string;
  description: string;
  status: string;
  startDate: string;
  endDate: string;
}

export interface Project {
  id: string;
  name: string;
  description: string | null;
  created_by_id: string;
  status: string;
  start_date: string | null;
  end_date: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface ProjectResolved {
  id: string;
  name: string;
  description: string | null;
  created_by_id: string;
  created_by: string;
  status: string;
  start_date: string | null;
  end_date: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export type CreateProjectResponse = CommonResponse<Project, FormError>;
export type UpdateProjectResponse = CommonResponse<Project, FormError>;
export type DeleteProjectResponse = CommonResponse<
  SimpleMessageData,
  SimpleError
>;
export type ProjectResponse = CommonResponse<Project, SimpleError>;
export type ProjectResolvedResponse = CommonResponse<
  ProjectResolved,
  SimpleError
>;
export type ProjectResolvedList = ProjectResolved[];
export type PaginatedProjectResolvedListResponse = PaginatedDataResponse<
  ProjectResolvedList,
  SimpleError
>;
