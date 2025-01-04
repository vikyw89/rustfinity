use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    // TODO: Implement logic here to return the median of the list
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let is_even = sorted_numbers.len() % 2 == 0;

    if !is_even {
        return sorted_numbers[(sorted_numbers.len() - 1) / 2] as f32;
    }

    let middle_left_index = sorted_numbers.len() / 2 - 1;
    let middle_right_index = sorted_numbers.len() / 2;
    let total = sorted_numbers[middle_left_index] + sorted_numbers[middle_right_index];
    return total as f32 / 2.0;
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let number_hashmap: HashMap<i32, i32> =
        numbers.iter().fold(HashMap::new(), |mut acc, &number| {
            *acc.entry(number).or_insert(0) += 1;
            acc
        });

    let max_count = number_hashmap.values().max().cloned().unwrap_or(0);

    let mut mode_numbers: Vec<i32> = number_hashmap
        .iter()
        .filter(|&(_, &count)| count == max_count)
        .map(|(&number, _)| number) // No need to dereference
        .collect();

    mode_numbers.sort();

    return mode_numbers;
}
