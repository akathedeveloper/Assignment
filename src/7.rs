fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    let arr = [12, 3, 4, 7, 5, 2, 10, 1];
    let k = 6;
    if let Some(kth_smallest) = kth_smallest_element(&arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Invalid value of k.");
    }
}
