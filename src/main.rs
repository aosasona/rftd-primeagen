// use std::{self, fs::read_to_string};

// fn iterator() {
//     let nums: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
//     println!("{:?}", nums);

//     // manual collect implementation
//     let data = vec![1, 2, 3]; // data will live until main returns
//     let mut nums2 = data.iter().map(|x| x + 1);
//     let mut new_nums2 = vec![];

//     while let Some(x) = nums2.next() {
//         new_nums2.push(x)
//     }

//     // using collect to join strings
//     let str_collect: String = ["hello", ", ", "world", "!"].into_iter().collect();
//     println!("{}", str_collect);
// }

// fn read_file() {
//     // read line and skip the even lines
//     let file = read_to_string("lines").unwrap();
//     file.lines()
//         .enumerate()
//         .filter(|(idx, _)| idx % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|(_, line)| println!("{}", line))
// }

// fn frargs() {
//     let target: String = std::env::args().nth(1).expect("no filename provided");
//     let file = std::fs::read_to_string(target).expect("unable to read file");
//     file.lines().for_each(|line| {
//         if let Ok(value) = line.parse::<usize>() {
//             println!("{}", value);
//         } else {
//             println!("line is not a number");
//         }
//     });
// }

struct SomeData {
    data: Vec<u8>,
    idx: usize,
}

impl Iterator for SomeData {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let current_idx = self.idx;
        self.idx += 1;

        return self.data.get(current_idx).map(|x| *x);
    }
}

impl SomeData {
    fn new(data: Vec<u8>) -> SomeData {
        SomeData { data, idx: 0 }
    }
}

fn main() {
    // iterator();
    // read_file();
    // frargs();
    let data: SomeData = SomeData::new(vec![1, 5, 10, 3]);
    for x in data {
        println!("{}", x);
    }
}
