extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive

#[derive(Serialize, Deserialize)]
struct Person{
    name: String,
    age: u8,
    is_male: bool
}

fn main(){
    let json_str = r#"
        {
            "name": "Mukundh",
            "age": 20,
            "is_male": true
        }

    "#;


    let res = serde_json::from_str(json_str)

    if res.is_ok(){
        let p: Person = res.unwrap();
        println!("the name is {}",p.name);
    }
    else{
        println!("NO!!!");
    }
}