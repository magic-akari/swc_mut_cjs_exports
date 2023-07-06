it("works", () => {
  const foo = [1, 2, 3];

  expect(foo[2]).toBe(3);
  expect(foo?.[2]).toBe(3);
});
