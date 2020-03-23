use std::collections::HashMap

fn main(){
    let mut marks = HashMap::new();

    //add values
    marks.insert("rust marks",96);
    marks.insert("JS marks",94);
    marks.insert("Py marks",92);
    marks.insert("C# marks",99);

    //find length
    let markslen = marks.len();

    //get values with keys
    match marks.get("JS"){
        Some(mark) => println!("you got {}", mark),
        None => println!("you did not read")
    }

    //removing value
    marks.remove("JS");

    //looping through Hashmap
    for(sub,mark) in &marks{
        println!("{}: {} marks",sub,mark )
    }

    //checking for key

    bool keythere = marks.contains_key("C++");

}
