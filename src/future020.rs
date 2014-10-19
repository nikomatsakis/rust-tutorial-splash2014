// Theme: Legal borrows and iterator invalidation.

pub fn main() {
    let mut vec: Vec<f64> = vec![22.0, 44.0, 66.0];

    vec.push(88.0);

    for (index, elem) in vec.iter().enumerate() {
        println!("Element `{}` is `{}`", index, elem);

        // vec.push(*elem);
    }

    vec.push(88.0);
}

// Exercise #1. What happens when you remove the commented out call to
// `vec.push` above? Why?
