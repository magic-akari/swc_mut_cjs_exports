export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    foo: ()=>foo,
    bar: ()=>bar
});
function foo() {
    foo = ()=>1;
    foo.bar = ()=>2;
    return 3;
}
let bar = function() {
    bar = ()=>1;
    exports.bar.bar = ()=>(0, exports.bar)();
    return 3;
};
