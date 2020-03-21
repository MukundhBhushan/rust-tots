fn main(){
    //replace
    { //represents code block not needed everytime
        let my_string = String::from("Rust is fantastic");
        println!("After replace {}",my_string.replace("fantastic", "great"));
    }

    //lines: splits sentence into indep words
    {
        let my_string = String::from("The weather is nice");

        for line in my_string.lines(){
            println!("{}",line);
        }
    }

    //split
    {
        let my_string = String::from("This+is+a+sentence");
        let token: Vec<&str> = my_string.split("+").collect() //collect() converts to vector
    }

    //trim: removes unwanted spaces.
    {
        let my_string = String::from("     My name is Muk    ");
        println!("After trim",my_string.trim())
    }

    //chars
    {
        let my_string = String::from("The weather is nice");
        match my_string.chars().nth(4){
            Some(c) => println!("{}",c),
            None => println!("No char at index")
        }
    }

    //slice
    {
        let my_string = String::from("Mukundh");
        let s1 = &my_string[0..4];
        let s2 = &my_string[5..7];

    }
}