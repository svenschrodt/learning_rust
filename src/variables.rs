/**
 * Public module testing basic usage of variables in Rust
 * 
 * 
 * Lesson goals: 
 * 
 * - By default variables in Rust are immutable 🤣 - it's a feature, we will see that l8er
 * - Variables are holding primitive data or references to data Variables 
 * - Rust variables are block-scoped {Rust in general is block-scoped}
 * - variable names in Rust are - *by convention* - named /under_scored/, rather than /camelCased/
 */
pub fn run() {

  // By default variables in Rust behave like a {'const'} assignment in ECMA script


  // Assignig string to variable (implicit type declaration)
  let first_name = "Sven";

  // this would cause an error {"cannot assign twice to immutable variable"}
  // first_name = "Foo";
  
  // Adding keyword  mut to assignment makes the var mutable
  let mut my_age = 49;
  
  // printing to STDOUT
  println!("My name is {} and I am {}", first_name, my_age);
  
  // reassign age (after 2020-12-09)

  my_age  = 50;
  println!("My name is {} and I am {}", first_name, my_age);

  // Defining a constant with explicit type declaration
  const ID: i32 = 1010;
  println!("ID: {}", ID);

  //  Defining a constant with explicit type declaration as float 
  const PI:f32 = 3.14;           

  println!("PI: {}", PI);

   // Defining a constant with explicit type declaration
   const CREATOR_INTERFACE: &str = "VCS@Linux@Thor";
   println!("CREATOR_INTERFACE: {}", CREATOR_INTERFACE);

  // Assigning multiple vars
  let ( my_name, my_age ) = ("Darth Bobo", 5000);
  println!("{} is {}", my_name, my_age );
}