# PlanetB SyntaxHighlighter
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/somehowchris/planetbb-syntax-highlighter)

## About

This is a small app, providing static files to have a frontend to format your code so you can paste it with styles to places like word documents.
> Copying rich text has some painpoints with firefox, if you're not a old grown firefox user and you experience pain, use a chromium powered browser like brave, edge or chrome

Visit [this github page](https://somehowchris.github.io/planetbb-syntax-highlighter/) to have a go. Everything stays inside your browser, if you are conserned about security and code leaking check out the [container seciton](#container) to host it yourself.


## Container

This app is also built as a `nginx-alpine` container image for `arm64` and `amd64` platforms. It's available on github container registry. The container will expose it static file server http interface on port __80__.

To use it:
```
docker run -p 8000:80 ghcr.io/somehowchris/planetb-syntax-highlighter:0.1.2
```
> This will start the container and open it to your network on port `8000`

## 🚴 For Developers

This app is built with rust and wasm-webpack. Dependent code such as the `SyntaxHighlighter` is copied from the google archive and as is in JS. If you would like to know more about it [head to this reamde page](./src/assets/js/highlighter/README.md)

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
