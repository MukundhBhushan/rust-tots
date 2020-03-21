fn main(){
    let mut x = 12;
    //multiple imutable references
    let xr = &x; //unmutable reference  
    let xr2 = &x; //unmutable reference  

    //only one mutable reference
    let dom = &mut x; //mutable reference 
    *dom +=1;

    println!("dom:{}",dom)
} 