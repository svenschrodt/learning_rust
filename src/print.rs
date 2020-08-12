/**
 * Public module  writing data to ŚTDOUT
 * 
 * @package learning_rust
 * @version 0.1
 * @since 2020-08-12
 * @author Sven Schrodt<sven@schrodt-service.net>
 * @see https://github.com/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @see https://travis-ci.org/github/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @copyright Sven Schrodt <sven@schrodt-service.net>
 */
pub fn run() {
    // Printing string literal to console
    println!("Hello world");
  
    // Basic Formatting with println
    println!("{} is from {}", "Sven", "Moers");
  
    // Positional Arguments
    println!(
      "I like {0} ,{0} and {0} - bad song {1} - Foo: {2}",
      "move it", "phew", 42
    );
  
    // Named Arguments
    println!(
      "{first}  {last} is a {activity} fan",
      first = "Sven",
      last = "Schrodt",
      activity = "Coding"
    );
  
    // Placeholder traits
    println!("Decimal: {} Binary: {:b} Hexadecimal: {:x} Octal: {:o}", 23, 23, 23, 23);
  
    // Placeholder for debug trait
    println!("{:?}", (1, 2, "third", "forth", "Perl", "Rust", true, '@'));
  
    // Basic mathematical expression's result 
    println!("23 + 23 = {}", 23 + 23);
  }