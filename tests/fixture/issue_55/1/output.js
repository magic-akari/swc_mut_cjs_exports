Object.keys(mod).forEach(function(key) {
    if (key === "default" || key === "__esModule") return;
    if (key in exports && exports[key] === mod[key]) return;
    exports[key] = mod[key];
});
import * as mod from "./someModule";
