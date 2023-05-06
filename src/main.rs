#![allow(unused)]

fn copy_vs_ownership() {
    // by default an assigment is a copy, so you can mutate an assigned value
    // without mutating the original value
    let mut a = [0; 5];
    let mut b = a;
    // a[1] = 5;

    // by default all variable values are passed by copy to a function as argument
    plus_one(a);
    println!("the value of a[2] is {}", a[2]);

    // a box is like a `unique_ptr` in C++, they hold values in the heap
    // and rust automatically deallocate the memory in the heap when the pointer goes out of scope
    let mut a = Box::new([0; 1_000_000]);
    // here we copy the pointer, without copying the value
    // but a transfer if ownership occurs and we cannot access or mutate the original value anymore
    // NOTE : OWNERSHIP only happens at compile-time, not at runtime
    let mut b = a;
    // cannot access or mutate the variable a because when we assigned it to b
    // the owner of the boxed value moved from a to b
    // a[1] = 2; // -> compiler error
    // println!("a[1] = {}", a[1]); // -> compiler error
}

// to mutate an argument of a function, add mut before
// it won't mutate the original value though
fn plus_one(mut x: [i32; 5]) -> i32 {
    // let mut x = x; // this is the equivalent of the line above
    x[2] = 12;
    x[2]
}

fn main() {
    // collections like String, Vec & HashMap use `Box` under the hood
    let first = String::from("Ferris");
    // by default if we pass `first` to `add_suffix`, the ownership of `first` is moved to `add_suffix`
    // so we can't use it after the move
    // to allow using `first` after the move, we have to clone
    // cloning copy the value without transfering ownership and every mutation to the cloned value
    // does not affect the original value
    let full = add_suffix(first.clone());
    println!("moved : {full}, original: {first}");
}
/**
* Ownership is primarily a discipline of heap management:

   - All heap data must be owned by exactly one variable.
   - Rust deallocates heap data once its owner goes out of scope.
   - Ownership can be transferred by moves, which happen on assignments and function calls.
   - Heap data can only be accessed through its current owner, not a previous owner.

*/
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    let x = 5;
    let y = 500;

    //
    name
}
