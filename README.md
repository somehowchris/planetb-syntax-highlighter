# PlanetB SyntaxHighlighter
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/somehowchris/plantetb-syntax-highlighter)

## About

This is a small app, providing static files to have a frontend to format your code so you can paste it with styles to places like word documents.
> Copying rich text has some painpoints with firefox, if you're not a old grown firefox user and you experience pain, use a chromium powered browser like brave, edge or chrome

Visit [this github page](https://somehowchris.github.io/plantetb-syntax-highlighter/) to have a go. Everything stays inside your browser, if you are conserned about security and code leaking check out the [docker seciton](#docker) to host it yourselfe.


## Docker

This app is also built as a `nginx-alpine` docker image for `arm64` and `amd64` platforms. It's available on docker hub as `chweicki/plantetb-syntax-highlighter`. The container will expose it static file server http interface on port __80__.

To use it:
```
docker run -p 8000:80 chweicki/plantetb-syntax-highlighter:0.0.2
```
> This will start the container and open it to your network on port `8000`

## 🚴 For Developers

This app is built with rust and wasm-webpack. Dependent code such as the `SyntaxHighlighter` is copied from the google archive and as is in JS. If you would like to know more about it [head to this reamde page](./src/highlighter/README.md)

### 🛠️ Build

When building for the first time, ensure to install dependencies first.

```
yarn install
```

```
yarn run build
```

### 🔬 Serve locally

```
yarn run dev
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.


