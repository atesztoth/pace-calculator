# nginx state for serving content
FROM nginx:alpine

# Set working directory to nginx asset directory
WORKDIR /usr/share/nginx/html

# Remove default nginx static assets
RUN rm -rf ./*

COPY docker/nginx-static.nginx.conf /etc/nginx/conf.d/default.conf

# Copy static assets from builder stage
COPY dist/apps/teacher-page/ .

# Containers run nginx with global directives and daemon off
ENTRYPOINT ["nginx", "-g", "daemon off;"]
