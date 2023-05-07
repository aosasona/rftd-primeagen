const foo = [1, 2, 3];
for (let i = 0; i < foo.length; i++) {
  foo[i] += 1;
}
console.log(foo);

// alternatively using map
const foo2 = [1, 2, 3].map((x) => x + 1);
console.log(foo2);
