export { };
Object.defineProperty(exports, "foo", {
    enumerable: true,
    get: ()=>foo,
    configurable: true
});
function foo() {
    exports.foo = ()=>1;
    exports.foo.bar = ()=>2;
    return 3;
}
