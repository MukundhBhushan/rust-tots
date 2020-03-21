struct Color(u8,u8,u8);

fn main(){
    let mut red = Color(255,0,0);
    println!("red: {}{}{}",red.0,red.1,red.2);
}