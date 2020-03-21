struct Person{
    name: String,
    age: u8
}

impl ToString for Person{
    fn to_string(&self)->String{
        return format!("My name is {}. I am {}",self.name,self.age);
    }
}

fn main(){
    let muk = Person{
        name: String::from("Mukundh"),
        age: 19
    };
    println!("{}",muk.to_string);
}
