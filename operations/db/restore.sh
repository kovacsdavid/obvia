#!/usr/bin/env bash
#
# This file is part of the Obvia ERP.
#
# Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

set -euo pipefail

readonly BACKUP_DIR="/path/to/backups"
readonly CONTAINER_NAME="your-postgres-container"

restore_db() {
  local restore_file="$1"
  local restore_path="$BACKUP_DIR/$restore_file"

  if [[ ! -d "$BACKUP_DIR" ]]; then
    printf 'Backup directory does not exist: %s\n' "$BACKUP_DIR" >&2
    return 1
  fi

  if [[ ! -f "$restore_path" || ! -r "$restore_path" ]]; then
    printf 'Backup is not a readable file: %s\n' "$restore_path" >&2
    return 1
  fi

  if docker exec -i "$CONTAINER_NAME" \
        psql -X -U postgres -d postgres \
        < "$restore_path"; then
    printf '%s has been successfully restored\n' "$restore_file"
  else
    printf '%s could not be restored\n' "$restore_file" >&2
    return 1
  fi
}

main() {
  if [[ $# -ne 1 ]]; then
    printf 'Usage: %s <backup-filename>\n' "$0" >&2
    return 1
  fi

  printf 'Restoring database...\n'
  restore_db "$@"

  printf 'Done!\n'
}

main "$@"
