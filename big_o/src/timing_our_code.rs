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

#[test]
fn test_sample() {
    let result = _add_up_to(6);
    let res = _add_up_too(6);
        
    assert_eq!(res, 21);
    assert_eq!(result, 21);
}