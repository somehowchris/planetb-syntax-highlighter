FROM rust:1.57.0-alpine as build

WORKDIR /src

COPY . .

RUN apk add --no-cache curl unzip bash

SHELL ["/bin/bash", "--login", "-c"]

RUN cargo install fnm
ENV PATH=/root/.fnm:$PATH

RUN fnm install && fnm use && npm i -g npm

RUN npm i -g yarn && yarn install

RUN yarn run build

FROM nginx:1.21.4-alpine
COPY --from=build /src/dist /usr/share/nginx/html