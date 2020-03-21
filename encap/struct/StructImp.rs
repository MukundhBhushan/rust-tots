struct Rectangle{
    width: u32,
    height: u32
}

//impl: used to define functions which are to be performed on the struct
//same name as the struct
impl Rectangle{
    fn print_desc(&self){ //self must be added to all impls. it helps in assesing vars in the struct
        println!("Rectangle {} X {}",self.width,self.height)
    }

    fn is_sq()->bool{
        return self.height == self.width;
    }
}


fn main(){
    let myRect = Rectangle{
        width:12,
        height:32


    myRect.print_desc();
    let sq = myRect.is_sq();
    
}