FROM node:17-alpine

ARG NPM_TOKEN

WORKDIR /app
COPY package.json package-lock.json .npmrc ./
RUN npm ci
COPY . .
ENV NODE_ENV=production
RUN npm run build

EXPOSE 9001
CMD ["npm", "start"]
