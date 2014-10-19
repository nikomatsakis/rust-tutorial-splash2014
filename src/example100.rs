// Theme: Defining iterators; named borrow scopes.

#[deriving(Show)]
struct List<T> {
    data: T,
    next: Option<Box<List<T>>>,
}

#[deriving(Show)]
struct ListIterator<'a, T:'a> {                        /*
                    ~~  ~~~~                            *
                    |    |                              *
                    |   Type of things in the list.     *
                    |                                   *
               Scope of references within the struct.   *
                                                        *
       The T:'a indicates that the type T must          *
       "outlive" the scope 'a (meaning: be valid for at *
       least the scope 'a). Required so that reference  *
       cannot outlive referent in type `&'a List<T>`.   */

    list: &'a List<T>,                 /*
          ~~~~~~~~~~~                   *
               |                        *
       List borrowed for the scope 'a.  */
}

pub fn main() {
    let mut x = List::new(66i);
    x = x.prepend(44);
    x = x.prepend(22);

    println!("List is {}", x);
    for (idx, elem) in x.iter().enumerate() {
        println!("Element #{} is `{}`", idx, elem);
    }
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

    fn iter(&self) -> ListIterator<T> {   /*
            ~~~~~                 ~~~      *
              |                    |       *
            Default is to use lifetime of  *
            the `self` reference.          */

        ListIterator {
            list: self
        }
    }
}

impl<'a,T> Iterator<&'a T> for ListIterator<'a,T> { /*
                    ~~~~~                            *
                      |                              *
               Iterator yields up                    *
               references with scope 'a.             */

    fn next(&mut self) -> Option<&'a T> {
        None // Um...
    }
}

// Exercise 1. Implement the iterator.

// Exercise 2 (extra credit). Implement an iterator over mutable references.
