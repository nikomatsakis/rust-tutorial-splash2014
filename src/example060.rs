// Theme: Traits and generic programming.

trait Numeric {                            /*
      ~~~~~~~                               *
         |                                  *
      Defines an interface called "Numeric" */

    fn zero() -> Self;                     /*
       ~~~~~~    ~~~~                       *
         |        |                         *
         |  The type for which the          *
         |  interface is defined            *
         |                                  *
       Free-standing function               */

    fn add(&self, other: &Self) -> Self;   /*
       ~~~ ~~~~~  ~~~~~~~~~~~~              *
        |   |         |                     *
        |   |    Argument of same type      *
        |   |    as the receiver            *
        |   |                               *
        |  Receiver is a reference          *
        |                                   *
      Method in the interface               */
}

pub fn main() {
    let ints: Vec<int> = vec![22, 44, 66];
    println!("Sum of `{}` is `{}`", ints, sum(&ints));

    let f64s: Vec<f64> = vec![0.5, 1.5, 2.5];
    println!("Sum of `{}` is `{}`", ints, sum(&f64s));
}

fn sum<N>(vec: &Vec<N>) -> N               /*
   ~~~~~~                                   *
     |                                      *
   Generic function over some type `N`      */

    where N : Numeric                      /*
    ~~~~~~~~~~~~~~~~~                       *
           |                                *
    Type N must implement                   *
    the trait Numeric                       */

{
    let mut sum: N = Numeric::zero();      /*
                     ~~~~~~~~~~~~~          *
                          |                 *
            Invoke a suitable version of    *
            zero() to produce a result of   *
            type `N`                        */

    for elem in vec.iter() {               /*
        ~~~~    ~~~~~~~~                    *
          |        |                        *
          |   Iterator over references      *
          |   to the elements (`&N`)        *
          |                                 *
        Each element will have              *
        type `&N`                           */

        let intermediate = sum.add(elem);  /*
                           ~~~~~~~~~~~~~    *
                               |            *
          Invoke a suitable version of      *
          `add` to add two instances of `N` */

        sum = intermediate;
    }

    sum
}

impl Numeric for int {              /*
~~~~ ~~~~~~~     ~~~                 *
 |      |         |                  *
 |      |      The type for which    *
 |      |      the trait is impl'd.  *
 |      |                            *
 |   The trait being implemented.    *
 |                                   *
Declares that we plan to implement   *
a trait for some type.               */

    fn zero() -> int { 0 }          /*
                 ~~~                 *
                  |                  *
        Return type of `Self`,       *
        tailored to this impl.       */

    fn add(&self, other: &int) -> int {
        *self + *other
    }
}

impl Numeric for f64 {
    fn zero() -> f64 { 0.0 }         /*
                 ~~~                  *
                  |                   *
        Return type of `Self`,        *
        tailored to _this_ impl.      */

    fn add(&self, other: &f64) -> f64 {
        *self + *other
    }
}

// Exercise 1. Add another impl for the type `u32`.
// Check that it works.

// Exercise 2. Why did we use an intermediate variable to store the
// result of adding each element and the previous sum? (Hint: it has
// to do with the borrowing rules.)

// Exercise 3. How might you modify the trait to avoid this
// intermediate?
//
// Hint i. `&mut`
// Hint ii. `Copy`
