pub fn run(){
    loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }
}