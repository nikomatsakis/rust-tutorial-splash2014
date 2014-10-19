// Theme: Returning references and borrow scopes.

#![feature(slicing_syntax)]         /*
~~ ~~~~~~~                           *
|    |                               *
|  Acknowledge use of incomplete     *
|  feature.                          *
|                                    *
Attach attribute to enclosing item.  *
In this case, the module.            */

pub fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = split_at(vec[], 5);
    println!("`{}` split at 5 yields `{}` and `{}`",
             vec, left, right);
}

fn split_at<'a>(slice: &'a [int], mid: uint) -> (&'a [int], &'a [int]) { /*
           ~~~~        ~~~                       ~~~        ~~~           *
            |           |                         |          |            *
          Scope         |                         |          |            *
                        |                         |          |            *
              Given a slice borrowed              |          |            *
              for scope 'a                   Produce two slices for       *
                                             scope 'a                     *
                                                                          *
       Think of them as "subleases".                                      */

    (slice[..mid], slice[mid..])
}

// Exercise 1. Try inserting various calls to `vec.push()` in
// `main()`. What happens? Does it make a difference where you insert
// the call? Discuss.

// Exercise 2. What happens if you take out all the references `'a`?
