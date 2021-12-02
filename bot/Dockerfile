FROM denoland/deno:alpine
USER deno
WORKDIR /bot

# Resolve and cache dependencies
COPY deps deps
RUN deno cache deps/*

# Build app
COPY . .
RUN deno cache bin/bot/main.ts

CMD ["deno", "run", "--allow-read", "--allow-env", "--allow-net", "bin/bot/main.ts"]
