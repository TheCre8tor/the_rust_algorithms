use std::collections::HashMap;

pub fn run() {
    /* ***************************************
    * Frequency Counter Pattern ->
    This pattern uses objects, sets, or hashmap to
    collect values/frequencies of values. it usually
    an O(N) time complexity.

    This can often avoid the need for nested loops
    or O(N^2) quadratic operations with arrays/strings.
     */

    /* Example No. 1:
    Write a function called 'same', which  accepts two
    arrays. The function should return true if every value
    in the array has it's corresponding value squared in
    the second array. The frequency of values must be the
    same.

    same([1, 2, 3], [4, 1, 9]) -> true
    same([1, 2, 3], [1, 9]) -> false
    same([1, 2, 1], [4, 4, 1]) -> false
    */

    let arr_1: Vec<u8> = vec![1, 2, 3, 2];
    let arr_2: Vec<u8> = vec![9, 1, 4, 4];

    let result = same(arr_1, arr_2);
    // println!("{}", result);
}

/// This function result to `O(3N)` or can be simplified to `O(N)` Time Complexity
fn same(arr_1: Vec<u8>, arr_2: Vec<u8>) -> bool {
    if arr_1.len() != arr_2.len() {
        return false;
    }

    let mut frequency_counter_1: HashMap<u8, u8> = HashMap::new();
    let mut frequency_counter_2: HashMap<u8, u8> = HashMap::new();

    for val in arr_1 {
        if frequency_counter_1.contains_key(&val) {
            let item = match frequency_counter_1.get(&val) {
                Some(data) => *data,
                None => 0,
            };

            frequency_counter_1.insert(val, item + 1_u8);
        } else {
            frequency_counter_1.insert(val, 1_u8);
        }
    }

    for val in arr_2 {
        if frequency_counter_2.contains_key(&val) {
            let item = match frequency_counter_2.get(&val) {
                Some(data) => *data,
                None => 0,
            };

            frequency_counter_2.insert(val, item + 1_u8);
        } else {
            frequency_counter_2.insert(val, 1_u8);
        }
    }

    for key in frequency_counter_1.keys() {
        // Check if keys correspond -->
        match frequency_counter_2.get_key_value(&key.pow(2)) {
            Some(_) => true,
            None => return false,
        };

        // Compare the values correspond -->
        let fc_value_1 = frequency_counter_2.get(&key.pow(2));
        let fc_value_2 = frequency_counter_1.get(key);

        if fc_value_1 != fc_value_2 {
            return false;
        };
    }

    // println!("{:#?}", frequency_counter_1);
    // println!("{:#?}", frequency_counter_2);

    true
}
