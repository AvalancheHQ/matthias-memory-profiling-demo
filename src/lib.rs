pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn rle_encode(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    let mut current = chars.next().unwrap();
    let mut count = 1;

    for c in chars {
        if c == current {
            count += 1;
        } else {
            use std::fmt::Write;
            let _ = write!(result, "{}{}", count, current);
            current = c;
            count = 1;
        }
    }

    use std::fmt::Write;
    let _ = write!(result, "{}{}", count, current);

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
