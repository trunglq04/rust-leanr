use std::fs::File;

fn main() {
    // exit program
    // panic!("Exit");
    // println!("Cannot access this code.");
    // let numbers = [1, 2, 3];
    // irrecoverable err
    // println!("Number at index 3: {}", numbers[3]);

    // Use File to open -> Result
    let data_result = File::open("data.txt");
    // unwrap -> get the content
    // yes -> get the value from file
    // no -> panic
    // match to unwrap
    // let data_file = match data_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the data file: {:?}", error),
    // };
    // println!("Data file {:?}", data_file);

    // Recover when no file exists
    // Type Result will return Ok(value) or Error
    let data_file = match data_result {
        Ok(file) => file,
        Err(_) => match File::create("data.txt") {
            Ok(new_file) => new_file,
            Err(error) => panic!("Problem creating a new data file: {:?}", error),
        },
    };

    println!("Data file: {:?}", data_file);

    let alice = get_user("Alice");
    match alice {
        Some(value) => println!("Value: {value}"),
        None => panic!("No value"),
    }

    // return str if exist
    let alice_unwrap = get_user("Bob").unwrap();
    println!("Bob unwrap: {}", alice_unwrap);

    let charlie_expect = get_user("Charlie").expect("Expect to have value");
    println!("Charlie expect: {}", charlie_expect);

    // CUSTOM ERR
    let file_path = "sample.txt";

    // Use match
    // Alter method for testing
    match read_file_contents2(file_path) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }


}

use std::io::Read;

fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    // Use `?` to unwrap to get value
    // because open() return Result
    let mut file = File::open(path)?;
    let mut contents = String::new();

    // Use `?` to unwrap to get value
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

// Định nghĩa custom error
#[derive(Debug)]
enum CustomError {
    FileOpenError,
    FileReadError,
}

// When using `?` the function return type must be in Result<T, Error>
// If apply custom error we have to map custom error to error type (case: unwrap to get value)
fn read_file_contents2(path: &str) -> Result<String, CustomError> {
    let mut file = File::open(path)
        .map_err(|_| CustomError::FileOpenError)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|_| CustomError::FileReadError)?;

    Ok(contents)
}

// Option mean return Some or None
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}
