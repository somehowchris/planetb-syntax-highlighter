const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const ClosurePlugin = require("closure-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    experiments: {
      asyncWebAssembly: true,
    },
    devServer: {
      //contentBase: distPath,
      compress: argv.mode === "production",
      port: process.env.PORT || 8000,
    },
    entry: "./bootstrap.js",
    optimization: {
      minimize: argv.mode === "production",
      minimizer: [
        new ClosurePlugin({mode: 'STANDARD', childCompilations: true}, {}),
      ],
    },
    output: {
      path: distPath,
      filename: "codestyle.js",
      webassemblyModuleFilename: "codestyle.wasm",
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ["style-loader", "css-loader", "sass-loader"],
        },
        {
          test: /\.css$/i,
          use: ["style-loader", "css-loader"],
        },
        {
          test: /\.(png|svg|jpg|jpeg|gif)$/i,
          type: 'asset/resource',
        },
        {
          test: /\.(woff|woff2|eot|ttf|otf)$/i,
          type: 'asset/resource',
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
    ],
    watch: argv.mode !== "production",
  };
};
