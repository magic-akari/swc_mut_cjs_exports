# [SWC plugin] workaround for jest

[![Test](https://github.com/magic-akari/jest_workaround/actions/workflows/test.yml/badge.svg)](https://github.com/magic-akari/jest_workaround/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/jest_workaround)](https://crates.io/crates/jest_workaround)
[![npm](https://img.shields.io/npm/v/jest_workaround)](https://www.npmjs.com/package/jest_workaround)

This is a SWC plugin to handle jest compatibility issues.

## usage

install

```bash
npm i jest_workaround
```

swcrc config

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["jest_workaround", {}]]
    }
  },
  "module": {
    "type": "commonjs"
  }
}
```

Make sure that `module.type` is `commonjs` since this plugin does not touch non-workaround parts, such as import statements.

## FAQ

1. When do I need this?

The swc-transformed CJS is compliant with the ESM specification. This means that exports is immutable.

I need to use swc to get transformed code which conforms to the ESM specification.
But I need to use jest in a CJS environment to test it.

The immutable exports is difficult to use for jest testing.
This plugin will transform the export statement into mutable exports.

2. Do I have a better choice?

Yes.

If I can run jest in an ESM environment, then I don't even need swc, or just use swc to transform TypeScript syntax.

There may be some issues with running jest in ESM, but they will be resolved over time.
Tracked by [facebook/jest#9430](https://github.com/facebook/jest/issues/9430).

Or, I don't need the behavior of ESM.
I can get the CJS behavior of exports by using the CJS syntax.

CJS specific syntax

```JavaScript
exports.foo = function foo(){
    return 42;
}
```

CTS(CJS in TypeScript) syntax

```TypeScript
export = {
    foo: function foo(){
        return 42;
    }
}
```

Notes:

- ESM style export means immutable exports when transformed into CJS
- ESM style import means hoisted require when transformed into CJS
