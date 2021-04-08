const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const HtmlPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./pkg/index.js",
    indexCss: "./web/citrus.scss",
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  module: {
    rules: [
      {
        test: /\.s[ac]ss$/i,
        use: [
          MiniCssExtractPlugin.loader,
          "css-loader",
          "sass-loader",
        ],
      }
    ],
  },
  plugins: [
    new CopyPlugin(
      {
        patterns: [
          {
            from: './img/*.png',
            to: dist,
            context: './web/',
          }
        ],
      }
    ),
    new HtmlPlugin({
      title: 'Citrus',
      meta: {
        "og:title": "Citrus",
        "og:description": "A 100% Orange Juice field editor.",
        "og:type": "website",
        "og:locale": "en_US",
      },
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
    new MiniCssExtractPlugin(),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
