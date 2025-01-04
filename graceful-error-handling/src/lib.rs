pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    let parsed_number = match input.parse::<i32>() {
        Ok(number) => number,
        Err(_) => return Err("Invalid input".to_string()),
    };

    if parsed_number < 0 || parsed_number > 100 {
        return Err("Percentage out of range".to_string());
    }

    Ok(parsed_number as u8)
}
