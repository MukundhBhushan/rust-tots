fn main(){
    let mut emp_str = String::new(); //empty string
    let mut my_string = String::from("hello i am Mukundh");

    //Length
    println!("{}",my_string.len());

    //is empty
    println!("{}",my_string.is_empty())

    //split by whitespace
    let mut words: Vec<String> = my_string.split_whitespace()
    for i in words{
        println!("{}",i);
    }

    //if string contains words
    println!("{}",my_string.contains("hello"));

    //concat strings 
    //string var must be "mutable"
    my_string.push_str("Bhushan.");
    println!("{}",my_string)

}


