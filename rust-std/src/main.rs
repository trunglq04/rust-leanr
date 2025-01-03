use std::collections::HashMap;

fn main() {
    // VECTOR
    // init empty vector
    let v: Vec<u8> = Vec::new();
    // init vector with values
    let mut v1: Vec<u32> = vec![5, 2, 3];
    v1.push(10);
    v1.push(20);
    println!("V1: {:?}", v1);

    let first = v1[0];
    let first_other = v1.first();
    println!("First: {}", first);
    println!("First using method first(): {:?}", first_other);
    let first_2 = v1.get(0);
    println!("First using method get(): {:?}", first_2);

    for item in &v1 {
        println!("Item: {}", item);
    }

    for item in &mut v1 {
        // &mut u32
        *item += 1;
    }
    println!("New v1: {:?}", v1);
    v1.remove(1);
    println!("v1 after removing: {:?}", v1);

    let mut colors = vec!["Red", "Yellow", "Green"];

    /*  These line of code will use and remove colors
    --------------------------------------------------
        for color in colors.into_iter() {
            print!("{} ", color);
        }
        println!("\n{:?}", colors);
    --------------------------------------------------
    */
    for color in colors.iter() {
        print!("{} ", color);
    }
    println!("\n{:?}", colors);

    for color in colors.iter_mut() {
        *color = "Pink";
    }
    println!("colors after modified: {:?}", colors);

    let numbers = vec![2, 3, 4];
    // Transforms an iterator into a collection.
    let res: Vec<i32> = numbers.iter().map(|x: &i32| x + 1).collect();
    println!("res: {:?}", res); // res: [3, 4, 5]
    
    // HASHMAP
    let mut user = HashMap::new();
    user.insert("username", "Alice");
    user.insert("nickname", "Car");
    println!("User:{:?}", user);
    
    println!("Username: {}", user["username"]);
    let username = user.get("username");
    println!("Username with get(): {:?}", username);

    for (key, value) in user.iter() {
        println!("Key: {key}, Value: {value}");
    }
    for item in user.iter() {
        println!("Key: {}, Value: {}", item.0, item.1);
    }

    user.insert("username", "Bob");
    let len = user.len();
    println!("len: {}", len);

    if user.contains_key("username") {
        println!("yes");
    } else {
        println!("no");
    }
    // New value updated for key "username"
    for item in user.iter() {
        println!("Key: {}, Value: {}", item.0, item.1);
    }
}
