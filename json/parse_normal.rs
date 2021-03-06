extern crate serde_json;
use serde_json::Value as JsonValue;


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
        let p: JsonValue = res.unwrap();
        println!("the name is {}",p["name"].as_str().unwrap());
    }
    else{
        println!("NO!!!");
    }
}