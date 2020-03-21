use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::open("info.txt").expect("cannot open");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Nope");

    println!("file contents:{}",content);
}