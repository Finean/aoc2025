# aoc2025
Advent of Code 2025 solutions.
All times exclude file read times.

| Day | Part 1 (ms) | Part 2 (ms) | Total (ms) |
|-----|--------|--------|-------|
| [1](#day-1)   | 0.027    | 0.046    | 0.073 |
| [2](#day-2)   | 0.008    | 1.35    | 1.36 |
| [3](#day-3)   | 0.026    | 0.12   | 0.15 |
| [4](#day-4)   | 0.53    | 0.99   | 1.52 |
| [5](#day-5)   | 0.076    | 0.038   | 0.11 |
| [6](#day-6)   | 0.060    | 0.12   | 0.18 |
| [7](#day-7)   | 0.065    | 0.085   | 0.13 |
| [8](#day-8)   | 5.1    | 0.94   | 6.04 |
| [9](#day-9)   | 0.094    | 11.0    | 11.1 |
| [10](#day-10)  | 0.31    | 212    | 212 |
| [11](#day-11)  | 0.075    | 0.108    | 0.18 |
| [12](#day-12)  | 0.090    |     | 0.090 |

# Walkthrough

## Day 1

### Part 1

Part 1 starts off fairly straightforward, all we need to do it add or subtract the number depending on the first character being 'L' or 'R' and then check whether our new value is 0 mod 100.

### Part 2 

Part 2 is slightly more complicated as we now need to consider every time the dial passes 0.

Suppose the Dial is pointing at 80 and we then get the instruction R236.

We can decompose R236 into 2 * 100 + 36 as the quotient and remainder when we divide by 100. 

We then know the dial will have passed 0 twice from the two full rotations. 

We can then add on 36 to our position counter and use :

`if (pos <= 0 || pos >= 100)` to test if our dial has then passed 0 an additional time.

**Warning!** If we start on 0 and have an input such as R200 our program will incorrectly add an additional rotation to our output accumulator, to fix this we can simply temporarily store our previous dial position and replace the previous if statement with: 

`if ((pos <= 0 || pos >= 100) && prev_pos != 0) && rem != 0`

Where rem is the remainder mod 100.

A slight caveat with doing it this way is we can't just use `pos %= 100` at the end of every loop to store our dial position mod 100 as rust allows this value in the range [-99, 99], so we do `pos += 100;` to ensure that our position always starts in the range [0, 99].

## Day 2

### Part 1

Part 1 starts off fairly simple. Let's start by considering an example input such as `933-1048`.

In this example we see that and number which is 3 digits long cannot be invalid, while of the 4 digit numbers in the range only 1010 is invalid.

We approach the problem like this:

For a given range X-Y we consider the possible lengths of numbers in the range, for `933-1048` we have [3, 4]. We can immediately ignore any odd values in this range. For the even lengths we start by considering the smallest possible invalid value (for 4 this is 1010) and then try every consecutive invalid value until we exceed the value of Y, e.g. `1010...1111` -> [terminates ` as `1111 > 1048`]. This finds every invalid value as we can construct these invali values in an increasing sequence from all the numbers (starting at 100...) of length `n / 2`. This allows us to find the number of invalid values in about 10μs.

We can optimise this approach for larger ranges such as [3, 4, 5, 6, 7] as we can easily calculate the number of invalid values of length 4 and 6 without testing whether are inside the range or not.

### Part 2

For part 2 we can use our a lot of our approach from part 1 to simplify things. Suppose we want to find all the invalid values of length 10. To do this we can consider all the prime factors of 10: 2 and 5, now we use our approach from part 1 to find every number with digits repeated 2 and 5 times.

With this approach we also need to be aware that some numbers may appear more than once, e.g. 111111 has digits repeated 1, 2, 3, and 6 times. By considering only considering prime divisors we significantly reduce the number of repeated values, but we can use a HashSet to quickly check whether a number has already been counted. Only ~5 numbers are reapted from our puzzle input using this approach.

## Day 3

### Part 1

The logic behind our approach to part 1 today is fairly straightforward. Let's use the example input `8853249223`. To make the largest possible joltage from this input we use the following rules:

1) We want our first (most significant) digit to be the largest possible value in the input - note that this cannot be the last digit in the input (we should prioritise earlier digits in the input here)
2) We want our second (least significant) digit to be the largest possible value after our first digit

To ensure that this gives us the maximum value we can simply use a for loop to go from left to right and check whether using this digit would give a higher joltage. 

### Part 2

For part 2 we now have 12 digits to choose from out input. We can use the same approach as we had for part 1. We prioritise the earlier, more significant, digits while also being aware that the ith digit (starting at 0 for the most significant) must be chosen from elements i..( input.len() - 12 - i) of our input.

## Day 4

### Part 1

To solve part 1 all we need to do is go through the input 1 by 1 and count the number of adjacent rolls of paper.

### Part 2

To solve part 2 we can use our code from part 1 and put it inside a loop which exits when we haven't removed a roll of paper in this iteration. We can optimise our approach slightly here by only checking squares adjacent to ones which changed during the last iteration (or all squares during our first).

## Day 5

### Part 1

Our approach for part 1 includes 2 parts:

1) We turn our set of fresh ingredient ID ranges into a sorted vector of non-overlapping ranges
2) We go through the list of available ingredient IDs and use a binary search on the sorted ranges to check whether our ingredient is fresh

This approach solves the problem in ~80μs, the main difficulties here are writing a binary search function which works on ranges, and merging possibly overlapping ranges in our sorted vector. This can be slightly optimised by merging the ranges [A, B] [B+1, C], as while they technically aren't overlapping there are no elements in between these ranges.

### Part 2

For part 2 we can then use our vector of sorted ID ranges to quickly find the answer.

**Warning!** The range [a, b] contains b - a + 1 elements. Ensure your vector of ranges is non-overlapping.

## Day 6

### Part 1

Day 6 is mainly a problem in parsing our input, for part 1 all we need to do is separate the individual numbers and correctly identify the last line of the input for our operation. Getting the answer is then straightforward.

### Part 2

It turns out we couldn't ignore the whitespace after all. Fortunately if we change how our function parses the input we can store the input digits in a Vector, and then take vertical slices of each line of vectors until we reach only whitespace to parse the numbers. The input also includes inconsistent choices between numbers starting and ending with whitespace so we need to make sure we're parsing this correctly as we calculate the sum.

## Day 7

### Part 1

For day 7 we construct the grid as a Vec<Vec<i32>>, by encoding a '.' as a 0 and a '^' as -1, then go line by line, checking whether the character below each beam is a '.' or a '^', when the beam splits we then set the adjacent points on the row below to have value 1. 

To get our output we then sum all the value on the bottom row.

### Part 2 

For part 2 we do exactly the same as in part 1, but instead of setting the value to 1 when the beam splits, we add the current value of the beam to the adjacent points on the row below.

## Day 8

Todo

## Day 9

Todo

## Day 10

Todo

## Day 11

Todo

## Day 12

Todo
