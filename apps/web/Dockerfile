FROM oven/bun:1.3.9-alpine AS build
WORKDIR /workspace

COPY package.json bun.lock /workspace/
COPY apps/web/package.json /workspace/apps/web/package.json
RUN bun install --frozen-lockfile

COPY apps/web /workspace/apps/web
WORKDIR /workspace/apps/web
RUN bun run build

FROM nginx:1.28.2-alpine
COPY apps/web/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /workspace/apps/web/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
