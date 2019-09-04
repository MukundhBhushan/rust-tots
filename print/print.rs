pub fn main() {
  // Print to console
    println!("Hello from the print.rs file"); //Hello from the print.rs file
    

    // Basic Formatting
    println!("{} is from {}", "Mukundh", "Lon"); //Mukundh is from Lon

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Mukundh", "Lon", "code"
    ); //Mukundh is from Lon likes to code

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    ); //John likes to play Baseball

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10); //Binary: 1010 Hex: a Octal: 12
    //":b" used to print as binary ":x" for hex ":o" for octal

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello")); (12, true, "hello")
    //":?"to print any type of value
    // Basic math
    println!("10 + 10 = {}", 10 + 10); //10 + 10 = 20
}