fn main() {
    println!("Hello, world!");
    // data types in Rust
    let integer: u32 = 42; // 32-bit signed integer
    let float: f64 = 3.14; // 64-bit floating point number
    let boolean: bool = true; // boolean type
    let character: char = 'R'; // character type
    let array: [char; 5] = ['R', 'u', 's', 't', '!']; // array of characters
    let array_mix: (i64, &str, bool, f64) = (12, "ahmed", true, 3.14);
    let slice1: &[i32] = &[1, 2, 3];
    let slice2: &[String] = &["ahmed".to_string(), "ali".to_string()];
    let slice3 = ["ahmed", "ali"];
    let slice4: &[&str] = &["ahmed", "ali"];
    let name : String = "reda ljaml".to_string();
    // println!("Integer: {}, Float: {}, Boolean: {}, Character: {}", integer, float, boolean, character);
    // println!("Array: {:?}", array);
    // println!("array is here {:?}",array_mix);
    // println!("slice is here ,{:?}",slice1);
    // println!("slice is here ,{:?}",slice2);
    // println!("slice is here ,{:?}",slice3);
    // println!("slice is here ,{}",slice4[1]);
    println!("name is here : {}",name);
    for (index, value) in slice4.iter().enumerate() {
        println!("slice element at index {} is : {}", index, value);
    }
}