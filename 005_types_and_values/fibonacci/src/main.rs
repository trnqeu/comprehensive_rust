// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n - 1) + saturating_add.fib(n - 2);
    }
}

fn main() {
    let n = 50;
    println!("fib({n}) = {}", fib(n));
}