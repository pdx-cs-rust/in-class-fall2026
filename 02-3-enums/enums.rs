fn main() {
    enum Test1 {
        Value1(i32),
        Value2(&'static str),
    }

    enum _Test2 {
        Value,
    }

    fn check(v: &Test1) {
        match v {
            Value1(x) => println!("yep {}", x),
            Value2(s) => println!("nope {}", s),
        }
    }


    use Test1::*;
    let v = Value1(5);
    check(&v);
    let v = Value2("me");
    check(&v);
}
