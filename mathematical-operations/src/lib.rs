pub fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
    // TODO: Return a tuple of 4 values: (sum, difference, multiply, divide)
    let sum = a + b;
    let difference = a - b;
    let multiply = a * b;
    let divide = a / b;
    return (sum, difference, multiply, divide)
}
