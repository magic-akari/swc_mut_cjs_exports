export function foo() {
  foo = () => 1;
  foo.bar = () => 2;
  return 3;
}
