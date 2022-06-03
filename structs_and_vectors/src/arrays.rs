pub fn big_o() {
    /* SECTION  Big O of Arrays
        1. Insertion -> it depends
        2. Removal -> it depends
        3. Searching -> O(N)
        4. Access -> O(1)
    
        When you need an ordering,
        arrays are excellent choice!
    */
    let names: Vec<&str> = vec!("Michael", "Melissa", "Andrea");

    println!("{:?}", names);
}