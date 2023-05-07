import fs from "fs";

const foo = [1, 2, 3];
for (let i = 0; i < foo.length; i++) {
  foo[i] += 1;
}
console.log(foo);

// alternatively using map
const foo2 = [1, 2, 3].map((x) => x + 1);
console.log(foo2);

fs.readFileSync("lines")
  .toString()
  .split("\n")
  .filter((_, idx) => idx % 2 == 0)
  .filter((_, idx) => idx > 1 && idx < 4)
  .forEach((line) => {
    console.log(line);
  });
