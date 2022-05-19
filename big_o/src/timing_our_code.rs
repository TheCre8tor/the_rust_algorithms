fn _add_up_to(n: u32) -> u32 {
    let mut total = 0;

    for i in 0..=n {
        total += i;
    }

    total
}

fn _add_up_too(n: u32) -> u32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let result = _add_up_to(6);
        
        assert_eq!(result, 21);
    }

    #[test] 
    fn should_return_21() {
        let result = _add_up_too(6);

        assert_eq!(result, 21);
    }
}
