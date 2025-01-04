pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    if from_unit == "C" && to_unit == "F" {
        Ok(value * 9.0 / 5.0 + 32.0)
    } else if from_unit == "F" && to_unit == "C" {
        Ok((value - 32.0) * 5.0 / 9.0)
    } else if from_unit == "C" && to_unit == "K" {
        Ok(value + 273.15)
    } else if from_unit == "K" && to_unit == "C" {
        Ok(value - 273.15)
    } else if from_unit == "F" && to_unit == "K" {
        Ok((value - 32.0) * 5.0 / 9.0 + 273.15)
    } else if from_unit == "K" && to_unit == "F" {
        Ok((value - 273.15) * 9.0 / 5.0 + 32.0)
    } else {
        Err("Invalid unit".to_string())
    }
}
