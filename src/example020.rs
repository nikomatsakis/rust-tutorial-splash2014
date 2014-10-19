// Theme: Ownership.

pub fn main() {
    let vec = vec![22, 44, 66]; /*
              ~~~~               *
               |                 *
       Another handy macro.      */

    let sum = sum(vec);         /*
              ~~~~~~~~           *
                |                *
              Function call (!)  */

    println!("The sum of the vector is `{}`", sum);
}

// Function declarations:

fn sum(v: Vec<int>) -> int {                        /*
~~     ~~~~~~~~~~~  ~~~~~~                           *
|           |         |                              *
Keyword     |         |                              *
        Arguments     |                              *
                   Return type                       *
                  (Default: "nil" ())                */

    let (mut i, c, mut sum) = (0, v.len(), 0); /*
        ~~~~~~~~~~~~~~~~~~~   ~~~~~~~~~~~~~~~   *
               |                     |          *
               |             Tuple of type      *
        Pattern matching     (int, int, int)    */

    while i < c {           /*
          ~~~~~              *
            |                *
         Look ma, no parens  */

        sum += v[i];
    }

    sum                      /*
    ~~~                       *
     |                        *
 Last expression in block     *
 is the "result" of the block */
}

// Exercise #1: Find and fix the bug.

// Exercise #2: Modify the `main` function to print both the vector
// contents and its sum (e.g., "The sum of `[22, 44, 66]` is
// `132`"). Why doesn't it compile? How can we modify `sum` to make
// this work?

// Exercise #3: Modify `sum` to compute the prefix sum instead and
// print the result.
