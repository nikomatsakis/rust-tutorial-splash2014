// Theme: Mutable borrowing.

use std::mem;                        /*
         ~~~                          *
          |                           *
   Import mem module into scope.      *
   http://doc.rust-lang.org/std/mem/  */

pub fn main() {
    let mut vec = vec![22, 44, 66];              /*
        ~~~                                       *
         |                                        *
       Must be mutable, we'll modify it in place. */

    let sum = prefix_sum(&mut vec);        /*
                         ~~~~~~~~           *
                            |               *
                Borrow the vector _mutably_ */

    println!("The prefix sum is `{}`, `{}`", vec, sum);
}

fn prefix_sum(v: &mut Vec<int>) -> int { /*
              ~~~~~~~~~~~~~~~~            *
                     |                    *
       Request a mutably borrowed vector  */

    let (mut i, c, mut sum) = (0, v.len(), 0);

    while i < c {
        let value = mem::replace(&mut v[i], sum);              /*
                    ~~~~~~~~~~~~ ~~~~~~~~~                      *
                         |           |                          *
                         |    Borrow individual vector element. *
                         |                                      *
           Helper function to overwrite a memory location       *
           and return old value.                                */

        sum += value;
        i += 1;
    }

    sum
}

// Walkthrough 1. Mutable borrows and aliasing.
