// Theme: Rust basics.

pub fn main() {
    let mut vec: Vec<int> = Vec::new();       /*
            ~~~  ~~~~~~~~   ~~~~~~~~~~~        *
             |      |            |             *
             |      |        Creates and       *
             |      |        returns a vector  *
             |      |                          *
             |   Optional                      *
             |   type annotation               *
             |                                 *
         Vector is owned                       *
         by this stack frame                   */

    /*
     * What does this look like at runtime?
     *
     * Stack                 Heap
     *
     * +----------+          +-----------+
     * | data     | -------> | element 0 |
     * | capacity |          | ...       |
     * | length   |          |           |
     * +----------+          +-----------+
     */

    vec.push(22);
    vec.push(44);
    vec.push(66);

    println!("Vector has length `{}` and contents `{}`", vec.len(), vec); /*
    ~~~~~~~~                    ~~~~              ~~~~   ~~~~~~~~~~~~~~    *
       |                         |                  |           |          *
    Macro that takes a          Placeholders -------+           |          *
    format string.                                          Arguments      */

    let string = format!("Vector has length `{}` and contents `{}`", vec.len(), vec);
    println!("{}", string);

} // <-- Here, `vec` goes out of scope, destructor will run and it will be freed.

// Exercise #1: Remove the type annotation. What happens? How can we fix it?
