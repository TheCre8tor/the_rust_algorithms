# Big-O Definition:

Big O notation is a mathematical notation that describes
the limiting behavior of a function when the argument tends
towards a particular value or totalinfinity.

# Big O Shorthands

1. Arithmetics operations are constant
2. Variables assignment is contant
3. Accessing elements in an array (by index) or in object (by key)
   is constant
4. In a loop, the complexity is the length of the loop
   times the complexity of whatever happens inside of
   the loop.

# Time Complexity

Time complexity describes the amount of computer time
it takes to run an algorithm.

# Space Complexity

Space Complexity of an algorithm is the total space taken
by the algorithm with respect to the input size.

# Auxiliary Space Complexity

Auxiliary Space Complexity is the space required by the
algorithm, not including space taken up by the inputs.

# Space Complexity in Rust

- Most primitives (bool, u8, i8, char etc...) are constant space
- Strings require O(n) space where n is the string length
- Compound types are generally O(n), where n is the length
  (for arrays) or the number of keys (for objects)
