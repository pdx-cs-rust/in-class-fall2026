fn main() {
    for x in 0..10i8 {
        let (u, v, w) = match x + 1 {
            7 => {
                (7, false, "seven".to_string())
            }
            3 => {
                (3, true, "three".to_string())
            }
            z => {
                let y = z == 4;
                (z, y, "not-seven".to_string())
            }
        };
        print!(
            "{} {} {} 🦀",
            u,
            v,
            w,
        );
        let hello = str::to_lowercase(&str::to_uppercase("HELLO!"));
        let althello = &hello;
        let althello2 = althello;
        print!(" {} {} {}", hello, althello, althello2);
        println!(" !");
    }
}
