FROM rust:1.56.0-slim as build

WORKDIR /src

COPY . .

RUN apt-get update -y && apt-get upgrade -y && apt-get install curl unzip -y

SHELL ["/bin/bash", "--login", "-c"]

RUN curl -fsSL https://fnm.vercel.app/install | bash
ENV PATH=/root/.fnm:$PATH

RUN eval "$(fnm env --shell=bash)"
RUN fnm install && fnm use

RUN npm i -g yarn && yarn install

RUN yarn run build

FROM nginx:1.21.4-alpine
COPY --from=build /src/dist /usr/share/nginx/html