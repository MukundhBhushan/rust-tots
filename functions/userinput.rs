use std::io;

fn main(){
    let mut input = String::new();
    print!("Enter");
    match io::stdin().read_line((&mut input))){
        Ok(_) => {
            print!("Yay: {}", input)
        }
        Err(e) => println!("NO")
    }
}