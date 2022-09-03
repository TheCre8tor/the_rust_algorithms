pub fn run() {
    /* ***********************************
      * Multiple/Two Pointers ->
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
      values that sum to zero or [] if a pair does name_two
      exist.

      sumZero([-3, -2, -1, 0, 1, 2, 3]) -> [-3, 3]
      sumZero([-2, 0, 1, 3]) -> []
      sumZero([1, 2, 3]) -> []
    */

    let result: Vec<i8> = sum_zero(vec![-4, -3, -2, -1, 0, 1, 2, 3, 10]);
    // let result: Vec<i8> = sum_zero(vec![-4, -3, -2, -1, 0, 5, 10]);

    println!("Sum Zero: {:?}", result);
}

/// This function result to `O(N)` Time Complexity, and `0(1)` Space Complexity
fn sum_zero(arr: Vec<i8>) -> Vec<i8> {
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1_usize;

    while left < right {
        let sum = arr[left] + arr[right];

        if sum == 0 {
            return vec![arr[left], arr[right]];
        } else if sum > 0 {
            right = right - 1;
        } else {
            left = left + 1;
        }
    }

    vec![]
}
