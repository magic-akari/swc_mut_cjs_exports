export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    foo: ()=>foo1,
    foofoo: ()=>foofoo,
    bar: ()=>bar1,
    "y-1": ()=>x1,
    ns: ()=>ns
});
import { foo } from "foo";
import { foo as foo1, foofoo as foofoo } from "foo";
import { bar } from "bar";
import { "b-a-r" as bar1 } from "bar";
import { "x-1" as x1 } from "baz";
import * as ns from "ns";
