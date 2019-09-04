pub fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    for i in numbers.iter_mut(){
        *i = *i*2;
    }
    println!("{:?}", numbers)
}