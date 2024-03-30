fn is_palindrome(s: &str) -> bool {
    let mut low = 0;
    let mut high = s.len() - 1;

    while low < high {
        if s.chars().nth(low) != s.chars().nth(high) {
            return false;
        }
        low += 1;
        high -= 1;
    }
    true
}

fn main() {
    let str = String::from("abbba");
    let str1 = String::from("abcded");

    println!("{} is palindrome {}", str, is_palindrome(&str));
    println!("{} is palindrome {}", str1, is_palindrome(&str1));
}
