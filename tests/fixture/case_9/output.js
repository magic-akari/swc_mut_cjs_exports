export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    SystemAlert: ()=>SystemAlert,
    configuration: ()=>configuration
});
const { Alert: SystemAlert  } = require("./components");
const { default: configuration  } = require("./configuration");
