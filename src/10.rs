fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let test_cases = vec![2, 3, 7, 13, 21, 29];
    
    for &num in &test_cases {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
