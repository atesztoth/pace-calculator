FROM node:iron-alpine3.18 as builder

RUN corepack enable
RUN corepack prepare pnpm@9.9.0

WORKDIR /app

COPY . .

RUN pnpm install --frozen-lockfile --prefer-offline
RUN pnpm build

FROM nginx:alpine as runner

WORKDIR /usr/share/nginx/html

# Remove default nginx static assets
RUN rm -rf ./*

# It is assumed the context is the frontend folder
# I don't know a solution now that would enable to me copy files from outside
# of the build context, and probably, there isn't.
# What I could do is copy the nginx config in the pipeline, but I have wasted enough time
# with build stuff like this, will come back to it later.
COPY ./nginx-static.nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /app/build/ .

# Containers run nginx with global directives and daemon off
ENTRYPOINT ["nginx", "-g", "daemon off;"]
