// Theme: Returning references and borrow scopes.

use std::collections::HashMap;

pub fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = split_at(vec[], 5);
    println("`{}` split at 5 yields `{}` and `{}`",
            vec, left, right);
}

fn split_at(slice: &[int], mid: uint) -> (&[int], &[int]) {
    //             ~~~~~~                 ~~~~~~  ~~~~~~
    //               |                      |       |
    //     Given a borrowed slice...        |       |
    //                             ...return back two subslices.

    (slice[..mid], slice[mid..])
}

// Exercise 1. Try inserting various calls to `vec.push()` in
// `main()`. What happens? Does it make a difference where you insert
// the call? Discuss.
