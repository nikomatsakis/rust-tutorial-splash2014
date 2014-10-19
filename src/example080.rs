// Theme: Memory allocation and ownership.

use std::fmt;

struct List<T> {
    data: T,
    next: Option<Box<List<T>>>,      /*
          ~~~~~~ ~~~                  *
            |     |                   *
            |  Pointer into the heap, *
            |  referent is owned      *
            |                         *
      Some or None, see below         */
}

/*
 * Basic building block:
 *
 * enum Option<T> {
 *    None,
 *    Some(T)
 * }
 *
 * Compiler recognizes Option<Box<T>>, and other similar enums, and
 * represents them using a null pointer.
 *
 * Hence List<T> looks like this:
 *
 * +------+
 * | data |
 * | next | --Some--> +------+
 * +------+           | data |
 *                    | next | -None-.
 *                    +------+
 */

pub fn main() {
    let mut x = List::new(44i);
    x = x.prepend(22);
    x = x.append(66);
    println!("x={}", x);
}

impl<T> List<T> {
    fn new(value: T) -> List<T> {
        List {
            data: value,
            next: None
        }
    }

    fn prepend(self, value: T) -> List<T> {
        List {
            data: value,
            next: Some(box self)
        }
    }

    fn append(self, value: T) -> List<T> {
        let next = match self.next {
            None => List::new(value),
            Some(list) => list.append(value),
        };

        List {
            data: self.data,
            next: Some(box next)
        }
    }
}

impl<T:fmt::Show> fmt::Show for List<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "[{}", self.data));

        let mut pointer = &self.next;
        loop {
            match *pointer {
                None => {
                    return write!(fmt, "]");
                }
                Some(box ref p) => {
                    try!(write!(fmt, ", {}", p.data));
                    pointer = &p.next;
                }
            }
        }
    }
}

// Exercise 1. Rewrite `append` to avoid reallocating the entire
// vector.

// Exercise 2. The methods `prepend` and `append` take ownership of
// the list. What is the downside of this? Can you rewrite `prepend`
// and `append` to take `&mut self` instead?
//
// Hint i. Consider `std::mem::swap`.
