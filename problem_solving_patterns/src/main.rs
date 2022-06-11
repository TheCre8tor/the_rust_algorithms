use regex::Regex;
use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");

    //? HOW DO YOU IMPROVE IN PROBLEM SOLVING?
    /* 1. Devise a plan for solving problems
    2. Master common problem solving patterns


    SECTION  1 -> Problem Solving Strategies.
    a. Understand the Problem.

       ! Little Exercise.
         Write a function which takes two numbers
         and returns their sum.

       - Can I restate the problem in my own words?
         * -> "implement addition"

       - What are the inputs that goes into the problem?
         * -> ints? | floats? | strings?

       - What are the outputs that should come from the
         solution to the problem?
         * -> ints? | floats? | strings?

       - Can the outputs be determined from the inputs?
         * -> Yes? | No?

       - How should I label the important pieces of data
         that are a part of the problem?
         * -> Function name: add
         * -> Args: num1 & num2
         * -> Result: sum

    b. Explore Concrete Examples.
       ? User Stories
       ? Unit Tests

       ! Little Exercise.
         Write a function which takes in a string and
         returns counts of each character in the string.

       - Start with simple examples.
       - Progress to more complex examples
       - Explore examples with empty inputs
       - Explore examples with invalid inputs

    c. Break It Down.
       fn char_count(input: &str) {
           1. make object to return at end.
           2. loop over string, for each char...
              -> if the char is a number/letter AND is a key in object, add one to count.
              -> if the char is a number/letter AND not in object, add it to object and set value to 1.
              -> if character is something else (space, period, etc.) don't do anything.
           3. return object at end.
       }

       ? invoke and response:
       * char_count("aaaa") -> { a: 4 }
       * char_count("hello") -> { h: 1, e: 1, l: 2, o: 1 }
       * char_count("Your PIN number is 1234") -> { 1: 1, 2: 1, 3: 1, 4: 1, b: 1, e: 1, 1: 2, etc... }

    d. Solve/Simplify.
       - Find the core difficulty in what you're trying to do
       - Temporarily ignore that difficulty
       - Write a simplified solution.
       - Then incorporate that difficulty back in.

    e. Look Back and Refactor.
       o- Refactoring Questions ->
       - Can you check the result?
       - Can you derive the result difficulty?
       - Can you understand it at a glance?
       - Can you use the result or method for some other problem?
       - Can you improve the performance of your solution?
       - Can you think of other ways to refactor?
       - How have other people solved this problem?

    */

    let response = char_count("Alexander1123443*&#".to_string());
    println!("{:?}", response);
}

// First Solution ->
fn char_count(name: String) -> HashMap<String, u8> {
    let mut object: HashMap<String, u8> = HashMap::new();

    let name = name.to_lowercase();

    let regex = Regex::new(r"[a-z0-9]").unwrap();

    for item in name.split("") {
        // Check if HashMap contains a key, if true increment it value.
        if object.contains_key(item) {
            let fount_item = *object.get(item).expect("");

            object.insert(item.to_string(), fount_item + 1);
        } else {
            // Definitely, it does not contains the key, check if key is
            // an alphanumeric char, if true set it value to 1
            if regex.is_match(&item) {
                object.insert(item.to_string(), 1);
            }
        }
    }

    object
}
