fn main() {

    // String Literal
    let name: &str = "Siddharth";
    println!("Name is {:?}", name);

    // Dynammic String
    let full_name: String = String::from("Siddharth Manjul");
    // Another way: let full_name: String = "Siddharth Manjul".to_string();
    println!("Full name is {:?}", full_name);
    println!("Full name stored in memory {:p}", &full_name);

    // String Slice
    let name_slice: &str = &full_name[10..16];
    println!("Sliced Name is {:?}", name_slice);

    // Vector
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('.');
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    println!("Removed character is {}", removed_char);

    println!("Now the array is {:?}", chars);

    // Creating a collection of letters
    chars.iter().for_each(|c| print!("{}", c));
    print!("\n");

    // Another way
    let new_chars: Vec<char> = vec!['h','e','l','l','o'];
    dbg!(&new_chars);

    let collected: String = new_chars.iter().collect();
    println!("Collected String is {}", collected);

    for c in new_chars {
        print!("{}", c);
        if c == 'o' {
            println!(", world!");
        }
    }
}