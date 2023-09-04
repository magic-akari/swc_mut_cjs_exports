const path = require("node:path");

const isDev = process.env.NODE_ENV === "development";

const plugin = isDev
  ? path.resolve(
      "./target/wasm32-unknown-unknown/debug/swc_mut_cjs_exports.wasm",
    )
  : path.resolve(
      "./target/wasm32-unknown-unknown/release/swc_mut_cjs_exports.wasm",
    );

module.exports = {
  transform: {
    "^.+\\.(t|j)sx?$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [[plugin, {}]],
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
  },
};
