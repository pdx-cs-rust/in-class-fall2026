fn f() -> usize {
    3
}

fn main() {
    const S: usize = 3;
    let a: [i32; S] = [2; S];
    let mut b = a;
    b[1] = -2;
    println!("{:?} {:?}", a, b);
    println!("{}", b[f()]);
}
