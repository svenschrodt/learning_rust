/**
 * Public module testing usage of alphabetic (alphanumeric data) types in Rust
 *  
 *  - Character type (char)
 *  - String
 *
 *
 * General:
 *
 * Character data type in Rust is the most primitive alphabetic type representing a (unicode) literal of a single character  
 *
 * String data type in Rust can be classified as:
 *
 *   - String Literal (&str) or
 *   - String Object (String)
 *
 *
 * @package learning_rust
 * @version 0.1
 * @since 2020-08-11
 * @author Sven Schrodt<sven@schrodt-service.net>
 * @see https://github.com/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @see https://travis-ci.org/github/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @copyright Sven Schrodt <sven@schrodt-service.net>
 */

pub fn run() {
   // Character type examples

   // Defining a character --> note the single quotes

   let why = 'w';
   println!("{}", why);
   let at = '@';
   println!("{}", at);
   // Defining a UNICODE character by its literal
   let savoring_food = '😋';
   println!("{}", savoring_food);
   // Defining a character by its UNICODE {U+1F923 => 'rolling on the floor laughing'}
   let rotfl = '\u{1F923}';
   println!("{}", rotfl);

   // Declaring strings by literals
   let location: &str = "Neverland";
   let name: &str = "Peter Pan";
   println!("{} is in {}", name, location);

   // By default string literals are static  
   
   // To explicitly specify the variable as static we use the follwing syntax

   let company:&'static str = "Rusty Consulting Ltd.";
   let location:&'static str = "London, UK";

   println!("company: {} location: {}",company,location);

   // The Standard Library provides a String obect type. Unlike string literals it is not a part 
   // of the core language. String (Object) is a growable collection. It is mutable and UTF-8 encoded
   // type. String object is allocated in the heap. 
   // String object offers many functions - @see https://doc.rust-lang.org/std/string/struct.String.html

   // Creating new (empty) string and getting length - decalring it as mutable
   let mut str1 = String::new();
   println!("String length is: {}",str1.len());

   // adding content to string
   str1.push_str("hello rusty world");
   println!("'{}'  - length is now: {}",str1, str1.len());

   // Creating new string and getting length
   let str2 = String::from("Learning_Rust");
   println!("length is {}",str2.len());

   // Converting string literal to string object
   
   let mut str3 = "I am an object!" .to_string();
   println!("{}",str3);

   // Adding string content
   str3.push_str(" - wow: really");

   // Adding char content
   str3.push('!');
   println!("{}",str3);
}
