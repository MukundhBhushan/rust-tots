// Arrays - Fixed list where elements are the same data types

use std::mem; //memory libraries

pub fn run() {
    let number = [1,2,3,4,5];
    let numbersFix: [i32; 4] = [1, 2, 3, 4]; //values cannot change
    let mut numbers: [i32; 4] = [1, 2, 3, 4]; //values can change but not added

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers); //print the array

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}