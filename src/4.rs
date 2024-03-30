fn main() {
    let n: u32 = 32; 
    
    let mut flag = false;
    if n == 0 || n == 1 {
        flag = true;
    }
    let mut i = 2;
    while i <= n / 2 {
        if n % i == 0 {
            flag = true;
            break;
        }
        i += 1;
    }
    if !flag {
        println!("{} is a prime number.", n);
    } else {
        println!("{} is not a prime number.", n);
    }
}
