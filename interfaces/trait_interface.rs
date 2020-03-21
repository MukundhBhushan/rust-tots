struct Person{
    name:String,
    age:u8
};

trait HasVoiceBox{
    //speak
    fn speak(&self);
    fn can_speak(&self)->bool;
}

impl HasVoiceBox for Person{
    fn speak(&self){
        println!("Hello my name is {}",Person.name);
    }
    fn can_speak(&self)->bool{
        if Person.age>0{
            return true;
        }return false;
        
        
    }
}

fn main(){
    let person = Person{
        name: String::from("Mukundh"),
        age: 19
    };

    println!("Can {} speak? {}", person.name, person.can_speak());
}
