// Example 100: Memory allocation and ownership.

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

// +------+
// | data |
// | next | --Some--> +------+
// +------+           | data |
//                    | next | -None
//                    +------+

// Basic building block:
//
// enum Option<T> {
//    None,
//    Some(T)
// }

pub fn main() {
    let mut x = List::singleton(44i);
    x = x.prepend(22);
    x = x.append(66);
    println!("x={}", x);
}

impl<T> List<T> {
    fn singleton(value: T) -> List<T> {
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
        self // obviously wrong
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

// Exercise 1: Write `append()`.
