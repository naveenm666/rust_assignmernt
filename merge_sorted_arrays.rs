use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    // Merge the two arrays while they both have elements
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged_array.push(arr1[i]);
            i += 1;
        } else {
            merged_array.push(arr2[j]);
            j += 1;
        }
    }

    // Add remaining elements from arr1, if any
    while i < arr1.len() {
        merged_array.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2, if any
    while j < arr2.len() {
        merged_array.push(arr2[j]);
        j += 1;
    }

    merged_array
}

fn main() {
    // Input first sorted array
    println!("Enter elements of the first sorted array separated by spaces:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let arr1: Vec<i32> = input1
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    // Input second sorted array
    println!("Enter elements of the second sorted array separated by spaces:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let arr2: Vec<i32> = input2
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    // Merge the two sorted arrays
    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged_array);
}
