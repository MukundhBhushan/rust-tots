fn main(){
    let res = is_even(10);
    println!("the number is {}",res);
}

fn is_even(no:u32)->bool{
    return no%2 == 0;
} 