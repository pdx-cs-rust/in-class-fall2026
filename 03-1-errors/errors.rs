use std::{fs, io::{self, Read}};

fn open_file() -> io::Result<fs::File> {
    let mut args = std::env::args();
    let filename = match args.nth(1) {
        Some(f) => f,
        None => return Err(io::Error::new(
            io::ErrorKind::Other,
            "invalid arguments",
        )),
    };
    fs::File::open(filename)
}

fn wc() -> io::Result<(usize, usize)> {
    let mut f = open_file()?;
    let mut buf = String::new();
    let nbytes = f.read_to_string(&mut buf)?;
    let nchars = buf.chars().count();
    Ok((nbytes, nchars))
}

fn main() {
    let (nbytes, nchars) = match wc() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    println!("{} bytes; {} chars", nbytes, nchars);
}
