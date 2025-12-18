# aoc2025
Advent of Code 2025 solutions.
All times exclude file read times.

| Day | Part 1 (ms) | Part 2 (ms) | Total (ms) |
|-----|--------|--------|-------|
| 1   | 0.027    | 0.046    | 0.073 |
| 2   | 0.008    | 1.40    | 1.41 |
| 3   | 0.048    | 0.14   | 0.19 |
| 4   | 0.53    | 0.99   | 1.52 |
| 5   | 0.076    | 0.038   | 0.11 |
| 6   | 0.060    | 0.12   | 0.18 |
| 7   | 0.065    | 0.085   | 0.13 |
| 8   | 5.1    | 0.94   | 6.04 |
| 9   | 0.094    | 11.0    | 11.1 |
| 10  | 0.31    | 212    | 212 |
| 11  | 0.075    | 0.108    | 0.18 |
| 12  | 0.090    |     | 0.090 |

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

**Warning** If we start on 0 and have an input such as R200 our program will incorrectly add an additional rotation to our output accumulator, to fix this we can simply temporarily store our previous dial position and replace the previous if statement with: 

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
