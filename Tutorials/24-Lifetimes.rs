#[allow(dead_code, unused_variables)]
pub fn example_0() {
    let r: &i32;

    let x: i32 = 10;
    let r = &x;

    println!("{}", r);
}

pub fn example_highest_age() {
    // Allocate space in Memory
    let highest_age: i32;

    // Initializing Variables
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;

    highest_age = finding_highest_age(&alice_age, &bob_age);

    // Printings
    println!("Highest age is {}", highest_age);

    fn finding_highest_age(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

// Lifetimes Example and fully based on Referencing.
pub fn lifetime_example_highest_age() {
    // Allocate space in Memory
    let highest_age: &i32;
    let final_age: i32;

    // Initializing Variables
    let alice_age: i32 = 40;

    {
        let bob_age: i32 = 21;

        highest_age = finding_highest_age(&alice_age, &bob_age);
        final_age = *highest_age;

        // Printings 
        // Solution 1
        // println!("Highest age is {}.", highest_age);
    }

    println!("Highest Final Age is {}.", final_age);

    fn finding_highest_age<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}
