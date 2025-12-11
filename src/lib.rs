pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn rle_encode(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    let chars: Vec<char> = input.chars().collect(); // Allocation just to iterate

    // REGRESSION: Buffer all runs in a vector before concatenating
    let mut runs: Vec<String> = Vec::new();

    let mut count = 1;
    let mut current = chars[0];

    for &c in &chars[1..] {
        if c == current {
            count += 1;
        } else {
            // REGRESSION: Pre-allocate with 10x capacity to avoid reallocations
            let mut run = String::with_capacity(count * 10);
            run.push_str(&format!("{}{}", count, current));
            runs.push(run);

            current = c;
            count = 1;
        }
    }

    // REGRESSION: Final run also gets buffered
    let mut final_run = String::with_capacity(count * 10);
    final_run.push_str(&format!("{}{}", count, current));
    runs.push(final_run);

    // Concatenate all runs
    for run in runs {
        result.push_str(&run);
    }

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
