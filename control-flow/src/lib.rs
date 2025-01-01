pub fn check_number_sign(number: i32) -> String {
    // Return `"positive"` if the number is positive.
    // Return `"negative"` if the number is negative.
    // Return `"zero"` if the number is zero.
    if number > 0 {
        return "positive".to_string();
    } else if number < 0 {
        return "negative".to_string();
    } else {
        return "zero".to_string();
    }
    // Step 1:
    // Check if the number is positive.

    // Step 2:
    // Check if the number is negative.

    // Step 3:
    // Handle the case where it's neither positive nor negative.
}
