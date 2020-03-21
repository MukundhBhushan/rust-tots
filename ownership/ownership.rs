fn main(){
    {
        x = 1; //x is the only owner of this specific "1"
    }
    // as x is not in the scope any longer the variable is droped
    if(x==1)
    {
        println!("x:{}",x)
    }
}