/**
 * Public module testing usage of enumeration types {'enum'}
 *  
 * @package learning_rust
 * @version 0.1
 * @since 2020-08-12
 * @author Sven Schrodt<sven@schrodt-service.net>
 * @see https://github.com/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @see https://travis-ci.org/github/svenschrodt/https://github.com/svenschrodt/learning_rust
 * @copyright Sven Schrodt <sven@schrodt-service.net>
 */

// Enums are data types containing a number of definite values


// Init Orientation values
enum Orientation {
    // Variants of (main) cardinal point
    North,
    East,
    South,
    West,
}

fn navigate(o: Orientation) {
    // Perform navigation action depending on orientation
    // @see https://doc.rust-lang.org/book/ch06-02-match.html for info on {'match'} control flow operator 
    match o {
        Orientation::North => println!("Navigating to north"),
        Orientation::East => println!("Navigating to east"),
        Orientation::South => println!("Navigating to south"),
        Orientation::West => println!("Navigating to west"),
    }
}

pub fn run() {

    // init variables with all tastes of Orientation
    let avatar1 = Orientation::North;
    let avatar2 = Orientation::East;
    let avatar3 = Orientation::South;
    let avatar4 = Orientation::West;

    // do navigation
    navigate(avatar1);
    navigate(avatar2);
    navigate(avatar3);
    navigate(avatar4);
}
