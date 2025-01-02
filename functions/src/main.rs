fn main() {
    greet();
    add();

    let a = 10;
    let b = 30;
    add1(a, b);

    let a = 20;
    let b = 40;
    add1(a, b);

    let mul = sub(a, b) + add3(a, b);
    println!("Mul:{mul}");

    let array = [1, 23, 432, 24, 88, 239];
    let res = find_max(&array);
    println!("Max Value: {res}");

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = find_events(&numbers);
    println!("Even numbers: {:?}", even_numbers);

    // Closure - anonymous func
    let my_func = || println!("This is my closure!");
    my_func();

    let my_func = |x| println!("This is my closure with value {}", x);
    my_func(10u32);

    let my_closure = |x, y| -> i32 { x + y };
    let res = my_closure(10, 11);
    println!("Res: {}", res);

    let res2 = my_closure(a, b);
    println!("Res: {}", res2);

    let array = [1, 2, 3, 4, 5, 6, 7];
    array.iter().for_each(|item| {
        println!("Each value in array closure: {}", item);
    });

    println!("{:?}", array.iter().find(|&&number| number % 3 == 0).unwrap());
    println!("{:?}", array.iter().position(|&number| number % 3 == 0).unwrap());

    array
        .iter()
        .enumerate()
        .for_each(|(index, number)| println!("Index number {} has number {}", index, number));
}

fn greet() {
    println!("Greet");
}

fn add() {
    let a = 10;
    let b = 20;
    let sum = a + b;
    println!("Sum: {sum}");
}

// With params
fn add1(a: i32, b: i32) {
    let sum = a + b;
    println!("Sum1: {sum}");
}

// With return value
fn sub(a: i32, b: i32) -> i32 {
    let res = a - b;
    res
}

fn add3(a: i32, b: i32) -> i32 {
    a + b
}

fn find_max(input: &[i32]) -> i32 {
    let mut max = input[0];

    for &num in input {
        if num > max {
            max = num;
        }
    }

    max
}

fn find_events(input: &[i32]) -> Vec<i32> {
    let mut events = Vec::new();

    for &num in input {
        if num % 2 == 0 {
            events.push(num);
        }
    }

    events
}
