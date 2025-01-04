pub fn factorial(n: u32) -> u128 {
    // Implement your code here
    // base case
    if n == 0 {
        return 1;
    }

    // recursive case
    return n as u128 * factorial(n - 1);
}
