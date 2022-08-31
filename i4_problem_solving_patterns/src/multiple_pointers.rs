pub fn run() {
    /* ***********************************
      * Multiple Pointers ->
      Creating pointers or values that correspond to an index
      or position and move towards the beginning, end or middle
      based on a certain condition.

      Very efficient for solving problems with minimal space
      complexity.
    */

    /* ***********************************
      * Example No. 1:
      Write a function called sumZero which accepts a sorted
      array or integers. The function should find the first pair
      where the sum is 0. Return an array that includes both
      values that sum to zero or undefined if a pair does name_two
      exist.

      sumZero([-3, -2, -1, 0, 1, 2, 3]) -> [-3, 3]
      sumZero([-2, 0, 1, 3]) -> undefined
      sumZero([1, 2, 3]) -> undefined
    */
}

/// This function result to `O(N)` Time Complexity, and `0(1)` Space Complexity
fn sumZero(arr: Vec<u8>) -> Vec<u8> {
    let left: usize = 0;
    let right: usize = arr.len() - 1_usize;
}
