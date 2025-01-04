pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    // Implement your code here
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
