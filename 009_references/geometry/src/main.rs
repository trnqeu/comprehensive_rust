// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.


fn magnitude(v: &[f64]) -> f64 {
    let mut magnified = 0.0;
    for elem in v {
        magnified += elem*elem;
    };
    magnified.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(v: &mut[f64]) {
    let mag = magnitude(v);
    for elem in v.iter_mut() {
        *elem /= mag;
    } 
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}