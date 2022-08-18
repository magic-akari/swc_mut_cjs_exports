export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    a: ()=>a,
    b: ()=>b,
    c: ()=>c
});
let a = function() {};
function b() {}
class c {
}
(0, exports.a)();
b();
new c();
let _ = {
    a: exports.a,
    b,
    c
};
a = function() {};
b = function() {};
(0, exports.a)``;
b``;
