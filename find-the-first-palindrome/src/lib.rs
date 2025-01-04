pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    // TODO: Implement the function here
    // handle start bigger than end
    let new_start = start.min(end);
    let new_end = end.max(start);

    for i in new_start..=new_end {
        if is_number_palindrome(i) {
            return Some(i);
        }
    }
    return None;
}

fn is_number_palindrome(n: i32) -> bool {
    // 2 pointers
    let mut start = 0;
    let mut end = n.to_string().len() - 1;
    while start < end {
        if n.to_string().chars().nth(start) != n.to_string().chars().nth(end) {
            return false;
        }
        start += 1;
        end -= 1;
    }
    return true;
}
