fn main(){
    println!("Occupation is {}", match get_option("Muk"){
        Some(o) => o,
        None => println("No person")
    })
}

fn get_option(name: &str) -> Option<&str>{
    match(name){
        "Muk" => Some("Software dev"),
        "Bhu" => Some("Doc"),
        _ => None
    }
}