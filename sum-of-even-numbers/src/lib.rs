pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    // Your code here...
    let mut sum = 0;
    for i in start..=end {
        if i % 2 == 0 {
            sum += i;
        }
    }

    return sum;
}
