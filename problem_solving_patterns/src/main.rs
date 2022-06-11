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
    */

    char_count("HiDear");
}

fn char_count(stri: &str) {
    let mut result: HashMap<&str, u8> = HashMap::new();

    println!("Array: {:?}", stri.split("").collect::<Vec<&str>>());

    for item in stri.split("") {
        if result.contains_key(item) {
            let search = *result.get(item).expect("");

            result.insert(item, search + 1);
        } else {
            if item != "" {
                result.insert(item, 1);
            }
        }
    }

    println!("{:?}", result.keys());
    println!("{:?}", result.values());
}
