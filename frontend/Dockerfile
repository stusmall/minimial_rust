# syntax = docker/dockerfile:1.4
FROM node:18.17.1 as build

COPY . /app
WORKDIR /app/
RUN --mount=type=cache,target=/root/.yarn YARN_CACHE_FOLDER=/root/.yarn yarn install --production
RUN yarn build

FROM nginx:1.24.0 as production
COPY --from=build /app/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]