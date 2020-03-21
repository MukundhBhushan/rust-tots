struct Color{
    red: u8,
    green:u8,
    blue: u8
};

fn main(){
    let mut Blue = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    print_color(&Blue);
}

fn print_color(c: &Color){
    println!("{}{}{}",c.red,c.green,c.blue);
}