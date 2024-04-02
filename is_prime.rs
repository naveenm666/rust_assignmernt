fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    
    // Check for divisibility from 2 to the square root of n
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false; // Found a divisor, not a prime number
        }
    }
    
    true // No divisors found, it's a prime number
}

fn main() {
    let num = 17; // Change the number to test here
    
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
