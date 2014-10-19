// Theme: Borrow scopes.

use std::fmt;

struct List<T> {
    data: T,
    next: Option<Box<List<T>>>,
}

pub fn main() {
    let mut x = List::new(66i);
    x = x.prepend(44);
    x = x.prepend(22);

    println!("Originally, x={}", x);

    {
        let data = x.data();
        println!("data={} x={}", data, x);
    }

    {
        let data = x.data_mut();
        *data += 1;
    }

    println!("after increment, x={}", x);
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

    fn data(&self) -> &T {
        //  ~~~~~     ~~
        //    |       |
        //  Implicit "sublease" -- given one
        //  borrowed value, return another that
        //  lasts for same time.

        &self.data
    }

    fn data_mut<'a>(&'a mut self) -> &'a mut T {
        //     ~~~~  ~~               ~~
        //      |    |                |
        //      |  Link input/output scopes, loans
        //      |  last the same time.
        //      |
        //   Explicitly declared scope

        &mut self.data
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

// Discuss 1: What happens when we remove the "extraneous" blocks?

// Discuss 2: What happens when we move the `println` inside of the
// `data_mut` block? Why?  What about if we try to call `data_mut`
// again.
