# Fibonacci Number Generator

A simple command-line Fibonacci number generator written in Rust.
Enter a position and the program prints the Fibonacci number at that position, then asks again until you choose to exit.

## How it works

The Fibonacci sequence starts 0, 1, 1, 2, 3, 5, 8 and so on, where each number is the sum of the two before it. This program counts from 1, so the 1st Fibonacci number is 0, the 2nd is 1, and the 10th is 34. Type a whole number from 1 to 94 to get its Fibonacci number, or type 0 to exit. Invalid input, such as letters, simply asks you again, and numbers above 94 are rejected because the result would be too big for a `u64`.

## What it demonstrates

This project practices functions with parameters and return values, `loop` with `break` and `continue`, `for` loops with ranges, mutable variables, `match` on a `Result` from `parse`, early returns, and reading user input with `std::io`.

## Requirements and how to run

You need Rust and Cargo installed (get them from https://rustup.rs). Clone the repository, move into the project folder, and run `cargo run`, then follow the on-screen prompts.

## Example

If you enter 5, the program prints `The 5 Fibonacci Number is 3`, and if you enter 10, it prints `The 10 Fibonacci Number is 34`.