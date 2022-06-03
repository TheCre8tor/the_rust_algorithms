pub fn big_o() {
    /* SECTION  Big O of Objects
    1. Insertion -> O(1)
    2. Removal -> O(1)
    3. Searching -> O(N)
    4. Access -> O(1)

    When you don't need any ordering,
    objects are an excellent choice!
    */

    #[derive(Debug)] 
    struct Instructor {
        first_name: String,
        is_instructor: bool,
        favourite_number: Vec<u8>,
    }

    let mut instructor = Instructor {
        first_name: String::from("Kelly"),
        is_instructor: true,
        favourite_number: vec![1, 2, 3, 4],
    };  

    // Insertion -->
    instructor.favourite_number.push(5);
    instructor.is_instructor = false;
    instructor.first_name = "Alexander".to_owned();
    
    println!("{:?}", instructor);
}