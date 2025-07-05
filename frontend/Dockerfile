FROM node:24-alpine as builder
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build

FROM httpd:2.4
COPY --from=builder /app/dist /usr/local/apache2/htdocs/
COPY ./httpd.conf /usr/local/apache2/conf/httpd.conf
EXPOSE 80
CMD ["httpd", "-D", "FOREGROUND"]
