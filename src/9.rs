fn main() {
    let mut str = String::from("racecar");
    println!("Original String: {}", str);
    let mut bytes = str.into_bytes();
    let len = bytes.len();
    for i in 0..len / 2 {
        bytes.swap(i, len - i - 1);
    }
    str = String::from_utf8(bytes).expect("Invalid UTF-8 sequence");

    println!("Reversed String: {}", str);
}
