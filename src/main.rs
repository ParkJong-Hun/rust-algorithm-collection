// Apply to entire file.
// [FYI] 'warnings' is the wild card.
#![allow(unused_imports)]
mod project_euler;
use project_euler::problem_1::problem_1_1;
use project_euler::problem_1::problem_1_2;
use project_euler::problem_2::problem_2_1;
use project_euler::problem_2::problem_2_2;
use project_euler::problem_3::problem_3;

// compile command is like `rustc foo.rs`
// run command is like `./foo`
fn main() {
    // it's not function, but macro.
    // so it must contains "!".
    // println!("Hello World!");

    // {:?} : debug print
    
    // println!("problem 1 answer: {}", problem_1_1());
    // println!("problem 1 answer: {}", problem_1_2());

    // println!("problem 2 answer: {}", problem_2_1());
    // println!("problem 2 answer: {}", problem_2_2());

    println!("problem 3 answer: {}", problem_3());
}
