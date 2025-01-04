pub fn describe_number(n: i32) -> String {
    // TODO: Implement the function here
    if n == 0 {
        return "Zero".to_string();
    }

    let is_positive = n > 0;
    let is_even: bool = n % 2 == 0;

    if is_even && is_positive {
        return "Positive even".to_string();
    } else if is_even && !is_positive {
        return "Negative even".to_string();
    } else if !is_even && is_positive {
        return "Positive odd".to_string();
    } else {
        return "Negative odd".to_string();
    }
}
