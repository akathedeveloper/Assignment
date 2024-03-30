fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let first_str = &strs[0];
    let mut prefix = String::new();
    
    'outer: for (i, ch) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
            if let Some(c) = s.chars().nth(i) {
                if c != ch {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(ch);
    }
    
    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);
}
