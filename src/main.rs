fn main() {
    let foo: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}", foo);

    // manual collect implementation
    let data = vec![1, 2, 3]; // data will live until main returns
    let mut foo2 = data.iter().map(|x| x + 1);
    let mut new_foo2 = vec![];

    while let Some(x) = foo2.next() {
        new_foo2.push(x)
    }
}
