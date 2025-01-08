fn main() {
    /*
        During the process of compiling code related to generics, the Rust Compiler will perform `monomorphization`
        Takes a long time to compile (because it creates many different versions that the program can use)
     */

    // dynamic type
    // T in this case got value 10 and datatype i32
    let x = Some(10);

    // T in this case is 10.0 and datatype f64
    let y = Some(10.0);

    // Generic type is a placeholder alter to other datatype in Rust

    // Reverse
    let x1 = OptionI32::Some(10);
    let y1 = OptionF64::Some(10.0);
    // Use specific fixed datatype

    let point1 = Point { x: 16f64, y: 16f64 };

    let pointGen = PointGen { x: 16f32, y: 16f32 };
    let pointGens = PointGens { x: 16f32, y: 16f64 };

    // Use generic type in function
    let res = bar(10u64);
    // Other usage
    let res2 = bar::<i32>(10);
    let res3 = bar::<f64>(10.0);
    // ::<T> -> turbo fish


}

// pub enum Option<T> {
//     None,
//     Some(T),
// }

pub enum OptionF64 {
    None,
    Some(f64),
}

pub enum OptionI32 {
    None,
    Some(i32),
}

pub struct Point {
    x: f64,
    y: f64,
}

pub struct PointGen<T> {
    x: T,
    y: T,
}

// Use multiple generic types 
pub struct PointGens<T, U> {
    x: T,
    y: U,
}

// T is generic type, placeholder -> stand for return value.
// E is Error -> stand for return err.
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


// define generic type in func
pub fn bar<T>(a:T) -> T {
    a
}


fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b;
    }
}
