fn main() {
    {
        let name = String::from("Hello");
    }

    // fruit1 is the owner of data type String
    let fruit1 = String::from("Banana");
    // fruit2 is the new owner of this string data
    // so fruit1 now got nothing and will be deleted
    let fruit2 = fruit1;

    println!("fruit2: {}", fruit2);

    let fruit = String::from("Apple");
    print_fruit(fruit);
}

fn print_fruit(str: String) {
    println!("str = {}", str)
}
