use std::collections::HashMap;

pub fn run() {
    let _result = anagram("azj", "aaz");

    println!("{}", _result);
}

fn anagram(name_one: &str, name_two: &str) -> bool {
    if name_one.len() != name_two.len() {
        return false;
    }

    let mut lookup_map: HashMap<&str, u8> = HashMap::new();

    for letter in name_two.split("") {
        let has_letter = match lookup_map.get(letter) {
            Some(item) => *item,
            None => 0,
        };

        if lookup_map.contains_key(&letter) && !letter.is_empty() {
            lookup_map.insert(letter, has_letter + 1);
        } else if !letter.is_empty() {
            lookup_map.insert(letter, 1);
        }
    }

    // for letter in name_two.split("") {
    //     let
    // }

    println!("{:?}", lookup_map);

    true
}
