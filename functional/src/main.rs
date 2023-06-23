use crate::smart_pointers::box_usages::start_use_box;

mod functional_features;
mod cargo_mores;
mod smart_pointers;


fn main() {
    // println!("Hello, world!");

    // run_closure();
    // run_closure2();

    // test_closure_borrow();

    // test_closure_mut_borrow();

    // test_closure_new_thread();

    // iterator_sum();

    start_use_box();
}
