pub fn run() {
    /* ***********************************
      * Sliding Window ->
      This pattern involves creating a window which can either be
      an array or number from one position to another.

      Depending on a certain condition, the window either increases
      or closes (and a new window is created)

      Very useful for keeping track of a subset of data in an array/
      string etc.
    */

    /* ***********************************
      * Example No. 1:
      Write a function called maxSubarraySum which accepts an array
      of integers and a number called n. The function should calculate
      the maximum sum of n consecutive elements in the array.

      maxSubarraySum([1, 2, 5, 2, 8, 1, 5], 2) -> 10
      maxSubarraySum([1, 2, 5, 2, 8, 1, 5], 4) -> 17
      maxSubarraySum([4, 2, 1, 6], 1) -> 6
      maxSubarraySum([4, 2, 1, 6, 2], 4) -> 13
      maxSubarraySum([], 4) -> null
    */

    let result = max_subarray_sum(vec![1, 2, 5, 2, 8, 1, 5], 2);
    println!("MAX: {}", result);
}

fn max_subarray_sum(arr: Vec<u8>, num: usize) -> u8 {
    let mut max_sum = 0;

    let mut temp_sum = 0;

    if arr.len() < num {
        return 0;
    }

    for idx in 0..num {
        max_sum += arr[idx];
    }

    temp_sum = max_sum;

    for idx in num..arr.len() {
        temp_sum = temp_sum - arr[idx - num] + arr[idx];
        max_sum = u8::max(max_sum, temp_sum);
    }

    max_sum
}
