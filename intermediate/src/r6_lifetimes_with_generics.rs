#[allow(dead_code, unused_variables)]
pub fn lifetime_with_generics() {
    // Allocate space in Memory
    let highest_age: &i32;
    let final_age: i32;

    // Initializing Variables
    let alice_age: i32 = 40;

    {
        let bob_age: i32 = 54;

        highest_age = finding_highest_age::<i32>(&alice_age, &bob_age);
        final_age = *highest_age;

        // Printings 
        // Solution 1
        // println!("Highest age is {}.", highest_age);
    }

    println!("Highest Final Age is {}.", final_age);

    fn finding_highest_age<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
struct Person<'p> {
    name: &'p str,
    points: &'p f32,
}

#[allow(dead_code, unused_variables)]
pub fn lifetimes_with_struct() {
    let highest_points: &f32;
    let final_points: f32;

    let alice = Person {name: "alice", points: &51.2};

    {
        let bob = Person {name: "bob", points: &52.1};

        highest_points = largest::<f32>(alice.points, bob.points);
        final_points = *highest_points;
    }

    println!("Highest Points are {}.", final_points);

    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}