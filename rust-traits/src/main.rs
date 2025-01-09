use std::fmt;

fn main() {
    let vios = Car {
        category: "Sedan".to_string(),
    };
    let speed_vios = vios.speed();
    println!("Vios speed: {speed_vios}");

    let kawasaki = Motorbike {
        category: "High-Speed Motor".to_string(),
    };

    let kawasaki2 = get_vehicle("Motorbike");

    let speed_kawa = kawasaki.speed();
    println!("Kawasaki speed: {speed_kawa}");

    print_vehicle_info(&kawasaki);
    print_vehicle_info(&vios);

    let bike = Bicycle {
        category: String::from("Giant"),
    };
    // Cannot run becauce Bicycle not impl Vehicle trait (interface).
    // print_vehicle_info(&bike);

    check_speed(&vios);
    check_speed2(&kawasaki);

    print_insurance_info(&vios);
    print_insurance_info2(&vios);

    // super trait
    display_info(&kawasaki);
    display(&vios);

    // TRAIT OBJECT
    // 2 types of Trait Object in use: Static Dispatch and Dynamic Dispatch
    let circle = Circle { radius: 10.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 10.0,
    };

    /*  - dyn Trait have what's called "dynamic sizing"
        - At compile time, Rust needs to know the exact size of every value
        - Different types implementing the same trait can have different sizes
        - Box provides a fixed-size pointer to heap-allocated data
    */
    // Using &dyn instead of Box<dyn>
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle];
    let vec: Vec<Box<dyn Drawable>> = vec![Box::new(circle.clone()), Box::new(rectangle.clone())];
    // ownership and remove
    // trait obj -> obj got the same adj with trait

    // use std::sync::Arc;

    // // Thread-safe shared ownership
    // let shapes: Vec<Arc<dyn Drawable>> = vec![Arc::new(circle), Arc::new(rectangle)];

    // STATIC DISPATCH
    draw_static(&circle);
    draw_static(&rectangle);

    // obj not implement trait Drawable
    let tri = Triangle {};
    // draw_static(&tri); // Error

    // DYNAMIC DISPATCH
    // execute when data of obj hasn't been acknowleged in runtime and decide by runtime (logical running phrase)
    // pointer of trait (trait obj)
    // keyword `dyn`
    // Dynamic dispatch
    println!("=================Dynamic Dispatch=================");
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle];

    draw_dynamic_vec(&shapes);
}

pub struct Bicycle {
    category: String,
}

pub struct Car {
    category: String,
}

// Implement for each object type
impl Car {
    fn get_category(&self) {
        println!("Category:{}", self.category);
    }
}

// Like INTERFACE in other langs
pub trait Vehicle {
    fn get_category(&self) -> String;
    fn speed(&self) -> u32;
}

impl Vehicle for Car {
    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn speed(&self) -> u32 {
        100
    }
}

pub struct Motorbike {
    category: String,
}

impl Vehicle for Motorbike {
    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn speed(&self) -> u32 {
        80
    }
}

// trait bound: only for type impl Vehicle (trait)
fn print_vehicle_info(vehicle: &impl Vehicle) {
    println!(
        "Category: {}, Speed: {}",
        vehicle.get_category(),
        vehicle.speed()
    );
}

fn check_speed<T: Vehicle>(vehicle: &T) {
    if vehicle.speed() > 80 {
        println!("{} is fast!", vehicle.get_category());
    } else {
        println!("{} is slow.", vehicle.get_category());
    }
}

// More in use
fn check_speed2(vehicle: &impl Vehicle) {
    if vehicle.speed() > 80 {
        println!("{} is fast!", vehicle.get_category());
    } else {
        println!("{} is slow.", vehicle.get_category());
    }
}

// Returning traits
// Because Rust doesn't know the exact size when returning trait -> Use Box<dyn Trait> -> Save to Heap
fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "Car" => Box::new(Car {
            category: String::from("Car"),
        }),
        _ => Box::new(Motorbike {
            category: String::from("Motorbike"),
        }),
    }
}

trait Insurable {
    fn insurance_name(&self) -> String;
}

// trait as param
fn print_insurance_info(item: &(impl Vehicle + Insurable)) {
    println!(
        "{} is insured by {}",
        item.get_category(),
        item.insurance_name()
    );
}

// trait Vehicle and Insurable must be implemented
fn print_insurance_info2<T: Vehicle + Insurable>(item: &T) {
    println!(
        "{} is insured by {}",
        item.get_category(),
        item.insurance_name()
    );
}

// FIX: impl Insurable for Car
impl Insurable for Car {
    fn insurance_name(&self) -> String {
        "Petrolimex".to_string()
    }
}

// All object type must impl Vehicle first
// Then It could imple Displayable
trait Displayable: Vehicle {
    fn display_info(&self) {
        println!(
            "Vehicle Category: {}, Speed: {} km/h",
            self.get_category(),
            self.speed()
        );
    }
}

impl Displayable for Car {}
impl Displayable for Motorbike {}
// Error: Bicycle not impl Vehicle
// impl Displayable for Bicycle {}

fn display_info<T: Displayable>(item: &T) {
    println!("{:?}", item.display_info());
}

fn display(item: &impl Displayable) {
    println!("{:?}", item.display_info());
}

// These could have very different sizes
#[derive(Clone)]
struct Circle {
    radius: f64, // 8 bytes
}

#[derive(Clone)]
struct Rectangle {
    width: f64,  // 8 bytes
    height: f64, // 8 bytes
}

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

// Impl trait Drawable for 2 structs
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.");
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Triangle {}

// static dispatch
// T like a trait obj which is contraint by `Drawable`
// use trait bound
// use trait as a param
fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}

fn draw_static_circle(shape: &Circle) {
    shape.draw();
}

fn draw_static_rec(shape: &Rectangle) {
    shape.draw();
}

fn draw_dynamic(shapes: &[&dyn Drawable]) {
    for shape in shapes {
        shape.draw();
    }
}

// Custom Error Type
#[derive(Debug)]
enum DrawError {
    InvalidShape(String),
    DrawingFailed(String),
}

impl fmt::Display for DrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrawError::InvalidShape(msg) => write!(f, "Invalid shape: {}", msg),
            DrawError::DrawingFailed(msg) => write!(f, "Drawing failed: {}", msg),
        }
    }
}

fn draw_dynamic_vec(shapes: &[&dyn Drawable]) {
    for shape in shapes {
        shape.draw();
    }
}
