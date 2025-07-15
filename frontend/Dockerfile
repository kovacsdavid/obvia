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
