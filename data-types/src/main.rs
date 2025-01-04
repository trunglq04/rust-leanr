fn main() {
    // ENUMERATION
    let up = Direction::Up;
    println!("Up: {:?}", up);
    // match ~ if/else ~ case
    match up {
        Direction::Up => println!("Robot go Up"),
        Direction::Down => println!("Robot go Down"),
        Direction::Left => println!("Robot go Left"),
        Direction::Right => println!("Robot go Right"),
    }

    let mood_now = Mood::NotBad;
    let level = match_mood(&mood_now);
    println!("Level: {level}");

    use Star::*;
    let starvec: Vec<Star> = vec![BrownDward, RedDwaft, YellowStar, RedGiant];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That start is pretty big."),
        }
    }
    println!("What about DeadStar? It's the number {}", DeadStar as u32);

    let my_vec = vec![get_number(-800), get_number(8)];

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number),
        }
    }

    // STRUCT
    // compare to Tuple: struct more readable and more in structure
    // let student = ("Alice", 10, "C");
    let mut alice = Student {
        name: String::from("Alice"),
        age: 24,
        class: String::from("Rust Bootcamp"),
    };
    let bob = Student {
        name: String::from("Bob"),
        age: 24,
        class: String::from("C"),
    };

    println!("Alice Age: {}", alice.age);
    println!("Alice Class: {}", alice.class);

    alice.age = 20;
    println!("Alice Age after changing: {}", alice.age);

    let m1 = MacBook {
        name: String::from("M1"),
        os: String::from("MacOS"),
        chip: String::from("M1"),
    };
    let m2 = MacBook {
        name: String::from("M2"),
        os: String::from("MacOS"),
        chip: String::from("M2"),
    };
    let mut m3 = MacBook::new();

    println!("M1: {:?}", m1);
    println!("M2: {:?}", m2);
    println!("M3: {:?}", m3);

    // Explain self and Self
    // self: an object created by struct
    // Self: struct itself

    // self
    let name_m1 = m1.get_name();
    println!("Name M1: {}", name_m1);
    // error
    // println!("M!: {:?}", m1);
    let os_m2 = m2.get_os();
    println!("OS M2: {}", os_m2);
    println!("M2: {:?}", m2);   // still valid

    m3.set_os(String::from("Sonoma"));
    println!("M3 new: {:?}", m3);

}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    let happy_level = match mood {
        Mood::Happy => 1,
        Mood::Sleepy => 2,
        Mood::NotBad => 3,
        Mood::Angry => 4,
    };

    happy_level
}

enum Star {
    BrownDward = 10,
    RedDwaft = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}

pub struct Student {
    pub name: String,
    pub age: u8,
    pub class: String,
}

// pub struct MacBook;

// impl MacBook {
//     fn get_name(&self) -> String {
//         "M1".to_string()
//     }
// }

#[derive(Debug)]
struct MacBook {
    name: String,
    os: String,
    chip: String,
}

impl MacBook {
    fn new() -> Self {
        MacBook {
            name: "M1".to_string(),
            os: "MacOS".to_string(),
            chip: "M1".to_string(),
        }
    }

    // Use `self`
    fn get_name(self) -> String {
        self.name
    }

    // Use `&self`
    fn get_os(&self) -> &String {
        &self.os
    }

    // Use `&mut self`
    fn set_os(&mut self, new_os: String) {
        self.os = new_os;
    }
}
