#![allow(dead_code)] // An attribute to hide warnings for unused code.

#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

struct Unit; // A unit struct.

struct Pair(i32, f32); // A tuple struct.

struct Point { // A struct with two fields.
    x: f32,
    y: f32,
}

// Structs can be used as fields of another struct.
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {

    // Create struct with field init shorthand.
    let name = String::from("Brooklyn");
    let age = 22;
    let brooklyn = Person{name, age};

    // Print debug struct.
    println!("{:?}", brooklyn);

    // Instantiate a Point.
    let point: Point = Point{x: 10.3, y: 0.4};

    // Access the fields of the point.
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one.
    let bottom_right = Point{x: 5.2, ..point};

    // bottom_right.y will be same as point.y because we used that field from point.
    println!("Second Point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding.
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too.
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct.
    let _unit = Unit;

    // Instantiate a tuple struct.
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct.
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct.
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
