FROM rust:1.56.0-alpine

WORKDIR /src

COPY . .

RUN yarn install

RUN yarn run build