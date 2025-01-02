use std::array;

fn main() {
    // IF ELSE
    let x = 0;
    if x > 0 {
        println!("{x} is greater than zero");
    } else if x == 0 {
        println!("{x} is zero")
    } else {
        println!("{x} is less than zero");
    }

    let number = 50;

    if number > 10 && number < 30 {
        println!("Valid");
    } else {
        println!("Invalid");
    }

    loop {
        println!("Hello");
        break;
    }
    let mut number = 10;
    loop {
        if number == 0 {
            break;
        }
        println!("Number is: {}", number);
        number -= 1;
    }

    // WHILE
    let mut number2 = 3;
    while number2 > 0 {
        println!("Number is: {number2}");
        number2 -= 1;
    }

    let array = [1, 2, 3, 4, 5];

    for item in array {
        println!("Item here for {item}");
    }

    let mut counter = 0;
    while counter < array.len() {
        println!("Item here while: {}", array[counter]);
        counter += 1;
    }

    let mut sum = 0;
    for i in 1..11 {
        sum += i;
        println!("Sum is: {}", sum)
    }

    for item in array.iter() {
        println!("Item for iter: {item}");
    }

    for item in array.into_iter() {
        println!("Item for into_iter: {item}");
    }

    println!("{:?}", array);

    let mut arr2 = [1, 2, 3, 4, 5];
    for item in arr2.iter_mut() {
        *item = *item * 2;
    }
    println!("{:?}", arr2)

    // BREAK N CONTINUE
    
}