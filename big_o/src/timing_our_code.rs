/// Time complexity O(n)
pub fn add_up_to1(n: u64) -> u64 {
    let mut total = 0;

    for i in 0..=n {
        total += i;
    }

    total
}

pub fn add_up_to2(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let result = add_up_to1(10000000);
        
        assert_eq!(result, 50000005000000);
    }

    #[test] 
    fn should_return_21() {
        let result = add_up_to2(100);

        assert_eq!(result, 5050);
    }
}
