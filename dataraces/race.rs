fn main() {
    let str1 = set_str();
    println!("str1: {}",str1);
}

fn set_str() -> &String {
    let s2 = String::from("Hello");

    return &s2;
}

//this throws an error as the variable s2 is droped after the function runs
//causing the refence to be null

fn main() {
    let str1 = set_str();
    println!("str1: {}",str1);
}

fn set_str() -> String {
    let s2 = String::from("Hello");

    return s2;
}
