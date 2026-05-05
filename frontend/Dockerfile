#
# This file is part of the Obvia ERP.
#
# Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

FROM --platform=$BUILDPLATFORM node:24-alpine AS builder
ARG VITE_GIT_COMMIT_HASH
ENV VITE_GIT_COMMIT_HASH=$VITE_GIT_COMMIT_HASH
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build

FROM httpd:2.4
COPY --from=builder /app/dist /usr/local/apache2/htdocs/
COPY ./httpd.conf /usr/local/apache2/conf/httpd.conf
EXPOSE 80
CMD ["httpd", "-D", "FOREGROUND"]
