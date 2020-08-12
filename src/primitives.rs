/**
 * Public module testing usage of primitive data types in rust, such as:
 * 
 * - Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits used for instance)
 * - Float: f32, f64, 
 * - Boolean,
 * - Character and
 * - String
 * 
 * @package learning_rust
 * @version 0.1
 * @since 2020-08-12
 * @author Sven Schrodt<sven@schrodt-service.net>
 * @see https://github.com/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @see https://travis-ci.org/github/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @copyright Sven Schrodt <sven@schrodt-service.net>
 */

 // Rust is a statically typed language
 // Variable types have to be known at compile time
 // The compiler can usually infer, what type was meant (see defaults below, for cases that are not unique)

pub fn run() {

    // Implicit declaration of variable as integer {default: "i32"} by assingnig
    let x = 23;
  
    // Implicit declaration of variable as flo {default: "i32"} by assingnig
    let y = 2.5;
  
    // Explicit type declaration and initialization
    let z: i64 = 987654321;
  
    // Finding maximum sizes for some data types
    println!("Max u8: {}", std::u8::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i64: {}", std::i64::MAX);
  
    // // Explicit type declaration and initialization for boolean
    let is_active: bool = true;
  
    // Getting boolean values from  expressions
    let is_greater: bool = 23 < 5;
  
    let is_lower: bool = 42 < 13;

    // Defining a character --> note the single quotes
    let letter = 'w';

    // Defining a character as UNICODE { U+1F923 => 'rolling on the floor laughing'}
    let rotfl = '\u{1F923}';
   
    //	Writing debug trait "{:?}" to STDOUT
  
    println!("{:?}", (x, y, z, is_active, is_greater, is_lower, letter, rotfl));
  }