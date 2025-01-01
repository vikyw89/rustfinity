pub fn fizz_buzz(num: u32) -> String {
    // TODO: Implement the FizzBuzz challenge
    if num % 3 == 0 && num % 5 == 0 {
        return "FizzBuzz".to_string();
    }
    if num % 3 == 0 {
        return "Fizz".to_string();
    }
    if num % 5 == 0 {
        return "Buzz".to_string();
    }
    return num.to_string();
}
