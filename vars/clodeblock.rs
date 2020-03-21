fn main(){
    let x=10;

    //code block
    {
        //have access to vars outside the block 
        let y = 20;
        println!("x:{},y:{}",x,y);
        
    }
    //will throw error as y has no access outside code block
    println!("y:{}",y);

}