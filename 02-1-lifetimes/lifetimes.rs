fn g(p: &mut u32) -> &u32 {
    *p += 1;
    p
}

fn f<'a>() -> &'a u32 {
    let mut i = 17;
    g(&mut i)
}

fn main() {
    let r = f();
    println!("{}", *r);
}
