export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    mod: ()=>mod,
    foo: ()=>foo,
    bar: ()=>bar,
    baz: ()=>baz
});
Object.keys(mod1).forEach(function(key) {
    if (key === "default" || key === "__esModule") return;
    if (key in exports && exports[key] === mod1[key]) return;
    exports[key] = mod1[key];
});
import * as mod from "./someModule";
import * as mod1 from "./someModule";
import { foo as foo, bar as bar, baz as baz } from "./someModule";
