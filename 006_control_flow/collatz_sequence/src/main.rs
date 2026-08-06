// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 0;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
        
    }
    length

  
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}