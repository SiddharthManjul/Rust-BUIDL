#[allow(dead_code)]
mod r1_enums;
mod r2_structs;
mod r3_traits;
mod r4_encapsulation;
mod r5_lifetimes;

use crate::r5_lifetimes::example_0;
use crate::r5_lifetimes::example_highest_age;
use crate::r5_lifetimes::lifetime_example_highest_age;

fn main() {
    example_0();
    example_highest_age();
    lifetime_example_highest_age();
}
