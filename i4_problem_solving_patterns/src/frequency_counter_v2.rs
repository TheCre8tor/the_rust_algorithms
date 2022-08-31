use std::collections::HashMap;

pub fn run() {
    let response = same(vec![1, 3, 2, 2, 2, 3], vec![1, 9, 4, 4, 4, 9]);

    println!("V2: {response}");
}

fn same(arr_1: Vec<u8>, arr_2: Vec<u8>) -> bool {
    if arr_1.len() != arr_2.len() {
        return false;
    }

    let mut storage = HashMap::<u8, u8>::new();

    for value in arr_2 {
        let in_momory = match storage.get(&value) {
            Some(item) => *item,
            None => 0,
        };

        if storage.contains_key(&value) {
            storage.insert(value, in_momory + 1);
        } else {
            storage.insert(value, 1);
        }
    }

    for value in arr_1 {
        let result = match storage.get(&value.pow(2)) {
            Some(item) => *item,
            None => 0,
        };

        if result != 0 {
            // println!("V2: {:?}", storage);
            storage.insert(value.pow(2), result - 1);
        } else {
            return false;
        }
    }

    // println!("Data: {:?}", storage);

    true
}
