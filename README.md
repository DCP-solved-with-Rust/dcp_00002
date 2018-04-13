# Daily Coding Problem: Problem #2

> Good morning. Here's your coding interview problem for today.
>
> This problem was asked by Uber.
>
> Given an array of integers, return a new array such that each element at index i
> of the new array is the product of all the numbers in the original array except
> the one at i.
>
> For example, if our input was [1, 2, 3, 4, 5], the expected output would be
> [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be
> [2, 3, 6].
>
> Follow-up: what if you can't use division?

## Solution

Solved with Rust 1.27.0 nightly-2018-04-07. https://rustup.rs/

After solving this I looked at how others had done it and I saw that
https://github.com/vineetjohn/daily-coding-problem/blob/4fc87d0d06a3e8969ee55ea494157e0bba2a0eae/solutions/problem_02.py
was a better solution than my own original solution so I then adapted
my solution accordingly.

If you are interested in having a look at my original solution, see
[main.rs at revision cd73455](https://github.com/DCP-solved-with-Rust/dcp_00002/blob/cd734556154e172e7578881f35a5d2ac43fcf0f1/src/main.rs).

### Usage

`cargo run` and provide the list of numbers on stdin separated by whitespace.

Example:

```
cargo run <<EOF
1 2 3 4 5
EOF
```

Output:

```
120 60 40 30 24
```

Exits with non-zero if any errors occur.
