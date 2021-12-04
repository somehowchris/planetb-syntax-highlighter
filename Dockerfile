FROM rust:1.57.0 as build
WORKDIR /src

RUN apt-get update -y && apt-get upgrade -y && apt-get install curl unzip -y

RUN /usr/local/cargo/bin/cargo install fnm

ENV PATH=/root/.fnm:$PATH

COPY . .

RUN eval `fnm env --shell=bash` && fnm install && fnm use

RUN eval `fnm env --shell=bash` && npm i -g yarn && yarn install

RUN eval `fnm env --shell=bash` && yarn run build

FROM nginx:1.21.4-alpine
COPY --from=build /src/dist /usr/share/nginx/html