const TEAM: &str = "KKR";

fn main() {

    println!("{} is the first team qualifying for Finals in IPL 2024", TEAM);

    // Stack
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // For loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
        } else {
            println!("{}", i);
        }
    }

    // Mutable Variable
    let mut z: i32 = 5;
    print!("z was {}.", z);
    z = 10;
    println!(" After mutating, z is {}!", z);

    // Float value
    let freezing_temp: f64 = -2.4;
    println!("Freezing Temperature is {}", freezing_temp);
    
    // Boolean Value
    let is_zero_remainder: bool = 10 % 2 == 0;
    println!("Is Zero remainder? {}", is_zero_remainder);

    // Character
    // Whenever working with Characters, always use single quotes or backticks, that's how rust know that it's a Character.
    let first_character_of_name: char = 'S';
    println!("The first Character of my Name is {}", first_character_of_name);

    let emoji_char: char = '😎';
    println!("Emoji Character is {}", emoji_char);

    // array of floats'
    let my_floats: [f32; 10] = [0.11; 10];
    println!("The float array is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("The new array is {:?}", my_floats_new);
}
