use std::collections::HashMap;

pub fn run() {
    let _result = anagram("azar", "aazy");

    println!("{}", _result);
}

fn anagram(name_one: &str, name_two: &str) -> bool {
    if name_one.len() != name_two.len() {
        return false;
    }

    let mut name_one_storage: HashMap<&str, u8> = HashMap::new();
    let mut name_two_storage: HashMap<&str, u8> = HashMap::new();

    for value in name_one.split("") {
        let exist_value = match name_one_storage.get(value) {
            Some(item) => *item,
            None => 0,
        };

        if name_one_storage.contains_key(value) && !value.is_empty() {
            name_one_storage.insert(value, exist_value + 1);
        } else if !value.is_empty() {
            name_one_storage.insert(value, 1);
        }
    }

    for value in name_two.split("") {
        let value_exist = match name_one_storage.get(value) {
            Some(item) => *item,
            None => 0,
        };

        if name_two_storage.contains_key(value) && !value.is_empty() {
            name_two_storage.insert(value, value_exist + 1);
        } else if !value.is_empty() {
            name_two_storage.insert(value, 1);
        }
    }

    for value in name_two_storage.keys() {
        match name_one_storage.get_key_value(value) {
            Some(_) => true,
            None => return false,
        };
    }

    true
}
