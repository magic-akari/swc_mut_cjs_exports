# [SWC plugin] workaround for jest

[![Crates.io](https://img.shields.io/crates/v/jest_workaround)](https://crates.io/crates/jest_workaround)
[![npm](https://img.shields.io/npm/v/jest_workaround)](https://www.npmjs.com/package/jest_workaround)

[![Test](https://github.com/magic-akari/jest_workaround/actions/workflows/test.yml/badge.svg)](https://github.com/magic-akari/jest_workaround/actions/workflows/test.yml)
[![with @swc/core@latest](https://github.com/magic-akari/jest_workaround/actions/workflows/cron.yml/badge.svg)](https://github.com/magic-akari/jest_workaround/actions/workflows/cron.yml)

This is a SWC plugin to handle jest compatibility issues.

This SWC plugin should be used with `@swc/jest`.

## usage

install

```bash
npm i -D jest @swc/core @swc/jest jest_workaround
```

```js
// jest.config.js
const fs = require("node:fs");

const swcrc = JSON.parse(fs.readFileSync(".swcrc", "utf8"));

// If you have other plugins, change this line.
((swcrc.jsc ??= {}).experimental ??= {}).plugins = [["jest_workaround", {}]];

module.exports = {
  transform: {
    "^.+\\.(t|j)sx?$": ["@swc/jest", swcrc],
  },
};
```

Make sure that `module.type` is `commonjs` in your `.swcrc` since this plugin
does not touch non-workaround parts, such as import statements.

## FAQ

#### 1. When do I need this?

If you're using the swc compiler to transform your code to comply with the ESM
specification, but you're also using Jest to test it in a CJS environment, you
may encounter issues due to the immutable issue of `exports`.

This plugin can help by transforming the `export` statements into mutable
`exports`.

#### 2. Do I have a better choice?

You may have other options depending on your specific needs:

- If you're able to run Jest in an ESM environment, you can use swc to transpile
  TypeScript/JSX syntax or downgrade JavaScript syntax without module
  conversion. Simply set the value of `module.type` to `es6` to achieve this.

- It's possible that some issues related to running Jest in an ESM environment
  will be resolved over time. Keep an eye on
  [facebook/jest#9430](https://github.com/facebook/jest/issues/9430) for
  updates.

- If you don't need the behavior of ESM specifically, you can stick with the CJS
  syntax to get the CJS behavior of `exports`.

These options may be worth considering before using this plugin.

CJS syntax

```JavaScript
exports.foo = function foo() {
  return 42;
};
```

CTS(CJS in TypeScript) syntax

```TypeScript
export = {
  foo: function foo() {
    return 42;
  },
};
```

Notes:

- ESM style export means immutable exports when transformed into CJS
- ESM style import means hoisted require when transformed into CJS

#### 3. After upgrading the plugin version, the changes have not taken effect.

This is a known issue. You could remove the Jest cache by running
[`jest --clearCache`](https://jestjs.io/docs/cli#--clearcache) as a workaround.
