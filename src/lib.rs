pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn rle_encode(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    let chars: Vec<char> = input.chars().collect(); // Allocation just to iterate
    let mut count = 1;
    let mut current = chars[0];

    for &c in &chars[1..] {
        if c == current {
            count += 1;
        } else {
            // "Churn": format! allocates a temp string for the number
            result.push_str(&format!("{}{}", count, current));
            current = c;
            count = 1;
        }
    }
    result.push_str(&format!("{}{}", count, current));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
