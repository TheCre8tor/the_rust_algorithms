mod anagram_assignment;
mod count_unique_values;
mod frequency_counter_v1;
mod frequency_counter_v2;
mod two_pointers_v1;
mod two_pointers_v2;

fn main() {
    /* HOW DO YOU IMPROVE IN PROBLEM SOLVING?
       1. Devise a plan for solving problems
       * 2. Master common problem solving patterns

       ? SOME PATTERNS...
         - Frequency Counter
         - Multiple Pointers
         - Sliding Window
         - Divide and Conquer
         - Dynamic Programming
         - Greedy Algorithms
         - Backtracking
         - Many more!...
    */

    frequency_counter_v1::run();
    frequency_counter_v2::run();
    anagram_assignment::run();
    two_pointers_v1::run();
    two_pointers_v2::run();
    count_unique_values::run();
}
