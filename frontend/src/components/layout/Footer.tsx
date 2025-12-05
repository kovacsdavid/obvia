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

export function Footer() {
  const gitRevision = import.meta.env.VITE_GIT_COMMIT_HASH;
  return (
    <footer className="border-t py-4 px-4 text-center text-sm text-muted-foreground">
      <a
        href="https://github.com/kovacsdavid/obvia"
        target="_blank"
        rel="noopener noreferrer"
        className="underline"
      >
        Forráskód
      </a>
      {gitRevision && " // "}
      {gitRevision}
    </footer>
  );
}
