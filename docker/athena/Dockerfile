FROM node:latest AS tailwind-builder
WORKDIR /app

COPY . .
WORKDIR /app/athena
RUN npm install && \
npx tailwindcss -i input.css -o assets/tailwind.css

FROM rust:latest AS dx-builder 
WORKDIR /app

COPY --from=tailwind-builder /app /app
RUN cargo install cargo-binstall && \
cargo binstall dioxus-cli  && \
dx bundle -p athena

FROM nginx:alpine AS runner
WORKDIR /app

COPY --from=dx-builder /app/target/dx/athena/release/web/public/ /usr/share/nginx/html/

EXPOSE 80 

CMD ["nginx", "-g", "daemon off;"]
