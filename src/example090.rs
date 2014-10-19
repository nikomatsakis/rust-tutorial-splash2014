// Theme: Using iterators.

use std::iter::AdditiveIterator;
//  ~~~~~~~~~  ~~~~~~~~~~~~~~~~
//    |               |
//    |   Using a trait brings its methods
//    |   into scope.
//    |
//  Lots of fun stuff in here.

pub fn main() {
    let vec1: Vec<int> = vec![22, 44, 66];
    let vec2: Vec<int> = vec![44, 66, 88];

    println!("sum of `{}` is `{}`",
             vec1, sum(vec1[]));

    println!("sum of `{}` is `{}`",
             vec2, sum(vec2[]));

    println!("`{}` dot `{}` is `{}`",
             vec1, vec2, dot_product(vec1[], vec2[]));
}

fn sum(ints: &[int]) -> int {
    let mut sum = 0;
    for &i in ints.iter() {          /*
    ~~  ~~    ~~~~~~~~~               *
    |   |         |                   *
    |   |    Iterates over references *
    |   |    into the vector (`&int`) *
    |   |                             *
    |  Pattern match and              *
    |  extract the int                *
    |                                 *
    The main form of loop in Rust     */

        sum += i;
    }
    sum
}

fn dot_product(vec1: &[int], vec2: &[int]) -> int {
    vec1.iter()                 // -> Iterator<&int>
        .zip(vec2.iter())       // -> Iterator<(&int,&int)>
        .map(|(&a, &b)| a * b)  // -> Iterator<int>
        //    ~~~~~~~~
        //       |
        // Closure: pattern match against the input,
        // extracting the `a` and `b` values.
        .sum()                  /* -> int        *
         ~~~                                     *
          |                                      *
        Method `sum` defined in AdditiveIterator */
}
