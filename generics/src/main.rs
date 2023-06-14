use crate::lifetime::{test_longest, test_longest2};

mod generics;
mod struct_generics;
mod traits;
mod lifetime;


fn main() {
    // println!("Hello, world!");

    // get_max();
    // get_max_v2();

    // test_largest_function();

    // test_generic_functions();

    // test_point();

    // test_use_trait();

    // test_dangling_references();

    test_longest();
    test_longest2();
}
