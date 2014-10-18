// Theme: Enums and matching.

const PI: f64 = 3.14159;

#[deriving(Show)]
struct Point {
    x: f64,
    y: f64,
}

#[deriving(Show)]
enum Shape {     /*
~~~~              *
  |               *
Enumerated set of *
possible variants */

    Circle(Point, f64),     /*
    ~~~~~~ ~~~~~~~~~~        *
      |        |             *
      |    Variant arguments *
      |                      *
    Variant name             */

    Rectangle(/* upper-left */ Point, /* lower-right */ Point)
}

pub fn main() {
    let origin = Point { x: 0.0, y: 0.0, };
    let unit = Point { x: 1.0, y: 1.0, };

    let mut shape = Circle(origin, 22.0);
    println!("Area of `{}` is `{}`", shape, shape.area());

    shape = Rectangle(origin, unit);
    println!("Area of `{}` is `{}`", shape, shape.area());
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {   /*
        ~~~~~            *
          |              *
        Identify variant */

            Circle(_, radius) => 2.0 * PI * radius,   /*
            ~~~~~~ ~  ~~~~~~     ~~~~~~~~~~~~~~~~~     *
              |    |    |               |              *
              |    |    |        Result in this case   *
              |    |    |                              *
              |    |  Extract the radius               *
              |    |  into a new variable              *
              |    |                                   *
              | Ignore the origin                      *
              |                                        *
            In the event this is a circle...           */

            Rectangle(ref ul, ref lr) => {            /*
                      ~~~                              *
                       |                               *
                 Create reference into the value       *
                                                       *
        +--------------+                               *
        | self: &Shape | -----> +------------------+   *
        |  ul: &Point  | ---+   | [Rectangle]      |   *
        |  lr: &Point  | -+ |-> | Point { x: f64   |   *
        +--------------+  |     |         y: f64 } |   *
                          |---> | Point { x: f64   |   *
                                |         y: f64 } |   *
                                +------------------+   */

                (lr.x - ul.x).abs() * (lr.y - ul.y).abs()
             }
        }
    }
}

// Exercise 1. Remove one of the variants above. What happens?
