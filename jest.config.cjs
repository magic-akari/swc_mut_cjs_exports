const path = require("node:path");

const isDev = process.env.NODE_ENV === "development";

const plugin = isDev ? path.resolve("./debug") : ".";

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
