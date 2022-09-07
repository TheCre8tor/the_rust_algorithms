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

    let data: Vec<i8> = vec![2, 6, 9, 2, 1, 8, 5, 6, 3];
    let _result = max_subarray_sum(data, 3);

    // println!("MAX: {}", result);
}

fn max_subarray_sum(arr: Vec<i8>, num: usize) -> i8 {
    let mut max_sum = -0;
    let mut temp_sum = 0;

    /* If the length of the array is lower
    than the specified num, return 0 */
    if arr.len() < num {
        return 0;
    }

    /* Sumed up the items in the array from 0..num length */
    for idx in 0..num {
        max_sum += arr[idx];
    }

    /* Assign the value in max_sum into temp_sum */
    temp_sum = max_sum;

    for idx in num..arr.len() {
        /* subtract first moving index value from temp_sum,
        add the item at the location of idx, and reassign it
        value to temp_sum. */
        // [2, 6, 9, 2, 1, 8, 5, 6, 3]
        //  -        +
        // [2, 6, 9, 2, 1, 8, 5, 6, 3]
        //     -        +
        // [2, 6, 9, 2, 1, 8, 5, 6, 3]
        //        -        +
        // [2, 6, 9, 2, 1, 8, 5, 6, 3]
        //           -        +
        // [2, 6, 9, 2, 1, 8, 5, 6, 3]
        //              -        +
        // [2, 6, 9, 2, 1, 8, 5, 6, 3]
        //                 -        +

        temp_sum = temp_sum - arr[idx - num] + arr[idx];

        max_sum = i8::max(max_sum, temp_sum);

        // if temp_sum > max_sum {
        //     max_sum = temp_sum;
        // }
    }

    max_sum
}
