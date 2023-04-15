# syntax = docker/dockerfile:1.4
FROM node:18.15.0 as build 

COPY . /app
WORKDIR /app/frontend
RUN --mount=type=cache,target=/root/.yarn YARN_CACHE_FOLDER=/root/.yarn yarn install --production
RUN yarn build

FROM nginx:1.23.4 as production
COPY --from=build /app/frontend/build /usr/share/nginx/html
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]