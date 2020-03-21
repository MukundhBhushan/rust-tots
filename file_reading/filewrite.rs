use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::create("output.txt").expect("Cannot be created");

    file.write_all(b"hello I am Mukundh").expect("cannot write to the file");

}