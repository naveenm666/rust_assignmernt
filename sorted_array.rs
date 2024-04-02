use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            // Check if the current element is the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                // Move to the left side of the array
                high = mid - 1;
            }
        } else if arr[mid] < target {
            // If target is greater, search right half
            low = mid + 1;
        } else {
            // If target is smaller, search left half
            high = mid - 1;
        }
    }

    None // If target is not found
}

fn main() {
    println!("Enter the sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    println!("Enter the target integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Not an integer");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
