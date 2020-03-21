#![allow(dead_code)]
enum Day{
    Mon, tue, wed, thr, fri
    sat, sun
}

impl Day{
    fn is_weekday(&self)->bool{
        match self=>{
            &Day::sat | &Day::sun => return false,
            _ => return true
        }
    }
}

fn main(){
    let d = Day::tue

    println!("is d a weekend {}", d::is_weekday())
}