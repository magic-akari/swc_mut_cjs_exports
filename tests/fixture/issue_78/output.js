export { };
Object.keys(mod).forEach(function(key) {
    if (key === "default" || key === "__esModule") return;
    if (key in exports && exports[key] === mod[key]) return;
    exports[key] = mod[key];
});
Object.defineProperty(exports, "foo", {
    enumerable: true,
    get () {
        return foo;
    },
    set (v) {
        foo = v;
    },
    configurable: true
});
import * as mod from "./someModule";
const foo = ()=>{};
