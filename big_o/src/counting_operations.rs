/* NOTE:
   Rather than counting seconds, which are so variable...
   
   Let's count the number of simple operations the computer
   has to perform! */

// Example 1:
/// Time complexity O(1), because it is always 3 operations
/// -> CONSTANT TIME
fn _add_up_to(n: u32) -> u32 {
    n * (n + 1) / 2
}

/* List of operations
   1. multiplication (*) operation
   2. addition (+) operation
   3. divition (/) operation

   EXPLANATION: 
   There are 3 Operations and it doesn't matter what (n) is,
   if (n) is 2, or if (n) is a billion, there's only 3 calculations
   that are happening.
 */

// Example 2:
/// Time complexity O(n), because its number of operation is
/// bounded by a multiple of (n). eg -> 10n
/// -> LINEAR TIME
pub fn _add_up_to1(n: u64) -> u64 {
    // 1 assignment
    let mut total = 0;

    for i in 0..=n {
        // The addition (+) is (n) operation.
        // The equal sign (=) is also (n) operaion.
        total += i;
    }

    total
}

/* Counting the operaions of this function can be really hard
   depending on what we count, the number of operations can be
   as low as (2n) or as high as (2n + 1 assignment).
   
   But regardless of the exact number, the number of operations
   grows roughly propotionally with (n)*/


// SECTION  2 ->
/// The Big O of the function in total can be
/// simplified to O(n) instead of O(2n)
/// -> LINEAR TIME
pub fn _count_up_and_down(n: u32) {
    println!("Going up!");

    // Time Complexity -> O(n)
    for i in 0..n {
        println!("{}", i);
    }

    println!("At the top!\nGoing down...");

    // Time Complexity -> O(n)
    for j in (0..n).rev() {
        println!("{}", j);
    }

    println!("Back down. Bye!");
}


