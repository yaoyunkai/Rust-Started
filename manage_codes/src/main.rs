use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // let plant = crate::garden::vegetables::Asparagus {};

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
