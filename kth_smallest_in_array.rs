use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // If k is out of range, return None
    }
    
    let mut sorted_arr = arr.to_vec(); // Create a mutable copy of the array
    sorted_arr.sort(); // Sort the array in ascending order
    
    Some(sorted_arr[k - 1]) // Return the kth smallest element
}

fn main() {
    println!("Enter the array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();
    
    println!("Enter the value of k:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Not an integer");

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
