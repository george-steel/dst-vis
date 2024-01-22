use std::{env, fs::File, io::Read, path::Path};

mod dst;
mod model;

fn main() {
    let args: Box<[String]> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified.");
        return
    }
    let path = Path::new(&args[1]);
    println!("My path is {}.", path.display());
    let mut file = File::open(&path).expect("unable to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("unable to read file");
    let pattern = dst::decode_dst(&buf, ROYGBIV);
    println!("{:#?}", pattern.len());
}

const ROYGBIV: &[[f32;3]]  = &[[1.0, 0.0, 0.0]];
