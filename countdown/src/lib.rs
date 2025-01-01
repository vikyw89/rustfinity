pub fn countdown(n: u32) -> Vec<u32> {
    // TODO: Implement the countdown using a while loop
    let mut value: i32 = n as i32;
    let mut countdown: Vec<u32> = Vec::new();

    while value > -1 {
        countdown.push(value as u32);
        value = value - 1;
    }

    return countdown;
}
