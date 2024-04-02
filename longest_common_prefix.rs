use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    let first_string = &strings[0];
    
    for (i, &c) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(char_at_i) = string.chars().nth(i) {
                if char_at_i != c {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(c);
    }
    
    prefix
}

fn main() {
    println!("Enter strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let strings: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest Common Prefix: {}", common_prefix);
}
