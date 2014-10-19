// Theme: Iterators and for loops.

pub fn main() {
    let vec = vec![22.0, 44.0, 66.0];
    println!("The sum of `{}` is `{}`", vec, sum(&vec));
}

fn sum(vec: &Vec<f64>) -> f64 {
    //           ~~~
    //            |
    //  We support IEEE floating point too. Wow.

    let mut sum = 0.0;

    for f in vec.iter() {                       /*
        ~    ~~~~~~~~~~                          *
        |        |                               *
        |    Expression that yields an iterator. *
        |                                        *
    Pattern to match against the values          *
    yielded by the iterator.                     */

        // Type of `f` is `&float`: reference into `vec`
        sum += *f;
    }

    sum
}

// Exercise #1. See the iterator docs at
// <http://doc.rust-lang.org/std/iter/>.  Can you modify `sum` to be
// `prefix_sum`, as before?

