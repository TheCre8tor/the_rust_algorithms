use std::collections::HashMap;

pub fn run() {
    let _result = anagram("aza", "aaz");
}

fn anagram(name_one: &str, name_two: &str) -> bool {
    if name_one.len() != name_two.len() {
        return false;
    }

    let mut name_one_storage: HashMap<&str, u8> = HashMap::new();
    let mut name_two_storage: HashMap<&str, u8> = HashMap::new();

    for value in 0..name_one.len() {
        println!("{}", value);
    }

    true
}
