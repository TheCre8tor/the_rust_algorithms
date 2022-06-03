/// The space complexity of this function is constant space 
/// -> O(1) space!
/// Because we are only assigning values to a primitive
/// data type.
pub fn _sum(arr: Vec<u8>) -> u8 {
    let mut total = 0;

    for i in 0..arr.len() {
        total += arr[i]; 
    }

    total
}

/// The space complexity of this function is constant space 
/// -> O(n) space!
/// Because we are assigning values to a compound data type.
pub fn _double(arr: Vec<u8>) -> Vec<u8> {
    let mut new_arr: Vec<u8> = Vec::new();
    
    for i in 0..arr.len() {
        new_arr.push(2 * arr[i]);
    }

    new_arr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_sum() {
        let num: Vec<u8> = vec![1, 2, 3, 4, 5];
    
        let result = _sum(num);
    
        assert_eq!(result, 15);
    }
    
    #[test]
    fn calculate_double() {
        let data: Vec<u8> = vec![1, 2, 3];
        let compare: Vec<u8> = vec![2, 4, 6];
    
        let result = _double(data);
    
        assert_eq!(result, compare);
    }
}
