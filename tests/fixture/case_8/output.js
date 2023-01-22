export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    Alert: ()=>Alert,
    Button: ()=>Button,
    useTheme: ()=>useTheme
});
const { Alert , Button  } = require("./components");
const { useTheme  } = require("./theme");
