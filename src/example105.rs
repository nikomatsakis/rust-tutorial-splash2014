// Example 105: More on borrowing.

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

    fn data(&self) -> &T {
        &self.data
    }

    fn data_mut(&mut self) -> &mut T {
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

// Exercise 1: Write `append()`.
