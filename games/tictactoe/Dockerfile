FROM node:16-alpine

ARG NPM_TOKEN

WORKDIR /app
COPY package.json package-lock.json .npmrc ./
RUN echo "//npm.pkg.github.com/:_authToken=${NPM_TOKEN}" >> .npmrc
RUN npm ci
COPY . .
ENV NODE_ENV=production
RUN npm run build

EXPOSE 9001
CMD ["npm", "start"]
