pub fn run() {
  let person: (&str, &str, i8) = ("Brad", "Mass", 37);
  let tup = (20,12,34);
  let tup2 = (20,"hello",false,(1,5,0));
  let (a,b,c) = tup;

  println!("{} is from {} and is {}", person.0, person.1, person.2);
  println!("{}",tup.0)
}