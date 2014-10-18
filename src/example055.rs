// Theme: Structs, operator overloading, and inherent methods.

use std::ops::Add;

#[deriving(Show)]              /*
~ ~~~~~~~~ ~~~~                 *
|    |      |                   *
|    |     Trait for `println!` *
|  Automatically implement      *
Annotation on next item         */
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub fn main() {
    let unit = Vec3 { x: 1.0, y: 1.0, z: 1.0 }; /*
               ~~~~                              *
                |                                *
            Struct literal                       */
    let right = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    println!("`{}` \u2a2f `{}` is `{}`", unit, right, unit.cross(&right));
    println!("`{}` + `{}` is `{}`", unit, right, unit + right);
}

impl Vec3 {              /*
~~~~~~~~~                 *
    |                     *
No trait, methods are     *
just "inherent" to `Vec3` *
type                      */

    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

}

// The trait `std::ops::Add` is used for the `+` operator:
//
//     pub trait Add<RHS,Result> {
//         fn add(&self, rhs: &RHS) -> Result;
//     }
//
// RHS is the "right-hand-side" type.
// Result is the "result" type.

impl Add<Vec3,Vec3> for Vec3 {
    fn add(&self, rhs: &Vec3) -> Vec3 {
        Vec3 { x: self.x + rhs.x,
               y: self.y + rhs.y,
               z: self.z + rhs.z }
    }
}

// Exercise 1: Implement the `std::ops::Mul` trait to implement dot
// product using the `*` operator. Or, if you prefer, make an inherent
// method `dot()` instead.
//
// Hint: dot-product is the sum-of-products (`(x*x)+(y*y)+(z*z)`).
