/**
 * Public module testing usage of sequence data types in rust, such as:
 *
 * - Tuple,
 * - Array and
 * - Sclice
 *
 * @package learning_rust
 * @version 0.1
 * @since 2020-08-12
 * @author Sven Schrodt<sven@schrodt-service.net>
 * @see https://github.com/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @see https://travis-ci.org/github/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @copyright Sven Schrodt <sven@schrodt-service.net>
 */
// For getting allocated memory (module from std lib)
use std::mem;

pub fn run() {
  // A tuple groups values from different types

  // defining a tuple as (string literal, string literal, integer 8 bits wide and bool)
  let person: (&str, &str, i8, bool) = ("Sven", "Schrodt", 49, true);

  println!(
    "{} {} and is {} years old - activity: {}",
    person.0, person.1, person.2, person.3
  );
  // Arrays in Rust are fixed size list with elements of same data types
  let mut my_numbers: [i32; 6] = [99, 88, 77, 33, 22, 11];

  println!("{:?}", my_numbers);

  // re-assigning 4. value {index is 0-based, as we would expect}
  my_numbers[3] = 23;
  println!("{:?}", my_numbers);

  // Get single value from array, addressing index
  println!("Single Value: {}", my_numbers[0]);

  // Get length of an array
  println!("Array Length: {}", my_numbers.len());

  // Let's have a look at its memory usage
  println!("Array occupies {} bytes", mem::size_of_val(&my_numbers));

  // Getting an array Slice
  let slice: &[i32] = &my_numbers[2..4];

  // @see https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html
  // starting_index is the first position in the slice -> as expected
  // ending_index is *one more* than the last position in the slice -> WTF  🤔

  println!("Slice: {:?}", slice);
}
