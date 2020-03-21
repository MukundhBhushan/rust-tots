let number = 2;

match number{
    1 => println!("it is 1"),
    2 => println!("it is 2"),
    10 | 11 => println!("it is 10 or 11"),
    12...20 => println!("it is in between 12 to 20"),
    _ => println!("default statement");
}

let name = "Muk";

match name{
    "kun" => println!("it is kun"),
    "Muk" => println!("it is Muk"),
    _ => println!("default statement");
}