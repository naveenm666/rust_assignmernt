fn is_palindrome(s: &str) -> bool {
    let s = s.trim().to_lowercase(); // Convert to lowercase and trim whitespace
    let reversed = s.chars().rev().collect::<String>(); // Reverse the string
    s == reversed // Check if the original string is equal to the reversed string
}

fn main() {
    let test_cases = vec!["radar", "hello", "A man a plan a canal Panama"];
    
    for &test_case in &test_cases {
        println!("Is '{}' a palindrome? {}", test_case, is_palindrome(test_case));
    }
}
