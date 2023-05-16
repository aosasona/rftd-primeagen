import fs from "fs";
import path from "path";

// const foo = [1, 2, 3];
// for (let i = 0; i < foo.length; i++) {
//   foo[i] += 1;
// }
// console.log(foo);

// // alternatively using map
// const foo2 = [1, 2, 3].map((x) => x + 1);
// console.log(foo2);

// fs.readFileSync("lines")
//   .toString()
//   .split("\n")
//   .filter((_, idx) => idx % 2 == 0)
//   .filter((_, idx) => idx > 1 && idx < 4)
//   .forEach((line) => {
//     console.log(line);
//   });


// file reader via args
function frargs() {
  try {
    const target = process.argv[2];
    if(!target) return console.error('no file specified');
    const filePath = path.join(__dirname, "..", target);
    if(!fs.existsSync(filePath)) return console.error('specified file does not exist');
    fs.readFileSync(filePath).toString().split('\n').forEach((line) => {
      if(isNaN(parseInt(line.trim()))) {
       return console.log('line is not a number');
      }
      console.log(line);
    });
  } catch(e) {
    console.error("error: " + (e as { message: string }).message)
  }
}

frargs();