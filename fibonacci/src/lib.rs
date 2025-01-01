pub fn fibonacci(n: u32) -> u32 {
    // TODO: Implement the Fibonacci sequence
    // base case
    if n == 0 {
        return 0 as u32;
    }

    if n == 1 {
        return 1 as u32;
    }

    // recursive case

    return fibonacci(n - 1) + fibonacci(n - 2);
}
