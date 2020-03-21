fn main() {
    let s1 = String::from("Hello");
    print_str(s1);
}

fn print_str(s1: String){
    println!("s1:{}",s1);
}

//this throws an  error because Strings dont impliment the copy function 
//therefore the refence must be passed instead

//refences helps us in using the values without taking ownership
fn main() {
    let s1 = String::from("Hello");
    print_str(s1);
}

fn print_str(s1: &String){
    println!("s1:{}",&s1);
}