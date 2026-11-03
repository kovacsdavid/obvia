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

backup_db() {
  local timestamp backup_file tmp_file

  if [[ ! -d "$BACKUP_DIR" ]]; then
    printf 'Backup directory does not exist: %s\n' "$BACKUP_DIR" >&2
    return 1
  fi

  timestamp=$(date -u +'%Y-%m-%d_%H-%M-%S_UTC')
  backup_file="$BACKUP_DIR/db_backup_full-$timestamp.sql"
  tmp_file=$(mktemp "$BACKUP_DIR/.db_backup_full-$timestamp.XXXXXX")

  # NOTE: pg_dumpall provides consistency per database, not one synchronized snapshot 
  # across all databases
  # WARNING: Exit status zero confirms the dump command succeeded—not that the backup is restorable.
  # Periodically test restoration!
  if docker exec "$CONTAINER_NAME" pg_dumpall -U postgres > "$tmp_file"; then
    mv -- "$tmp_file" "$backup_file"
    printf 'Backup saved: %s\n' "$backup_file"
  else
    rm -f -- "$tmp_file"
    return 1
  fi
}

main() {
  printf 'Backing up database...\n'
  backup_db
  printf 'Done!\n'
}

main "$@"
