fn main() {
    let nums: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}", nums);

    // manual collect implementation
    let data = vec![1, 2, 3]; // data will live until main returns
    let mut nums2 = data.iter().map(|x| x + 1);
    let mut new_nums2 = vec![];

    while let Some(x) = nums2.next() {
        new_nums2.push(x)
    }

    // using collect to join strings
    let str_collect: String = ["hello", ", ", "world", "!"].into_iter().collect();
    println!("{}", str_collect);
}
