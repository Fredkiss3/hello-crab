#![allow(unused)]

fn copy_vs_ownership() {
    // by default an assigment is a copy, so you can mutate an assigned value
    // without mutating the original value
    let mut a = [0; 5];
    let mut b = a;
    a[2] = 5;

    // by default all variable values are passed by copy to a function as argument
    plus_one(a);
    println!("the value of a[2] is {}", a[2]);

    // a box is like a `unique_ptr` in C++, they hold values in the heap
    // and rust automatically deallocate the memory in the heap when the pointer goes out of scope
    let mut a = Box::new([0; 1_000_000]);
    // here we copy the pointer, without copying the value
    // but a transfer of ownership occurs and we cannot access or mutate the original value anymore
    // NOTE : OWNERSHIP only happens at compile-time, not at runtime
    let mut b = a;
    // cannot access or mutate the variable a because when we assigned it to b
    // the owner of the boxed value moved from a to b
    // a[1] = 2; // -> compiler error
    // println!("a[1] = {}", a[1]); // -> compiler error

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

// to mutate an argument of a function, add mut before
// it won't mutate the original value though
fn plus_one(mut x: [i32; 5]) -> i32 {
    // let mut x = x; // this is the equivalent of the line above
    x[2] = 12;
    x[2]
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

fn reference_v1() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // You pass references to functions by using `&` before the variable
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    println!("Formatted `{}`", s);
}

/**
 * A reference, marked with `&` is a pointer that do not own the value of the variable it references
 * the reference do not deallocate the original data when the variable goes out of scope,
 * How it works :
 *  - &g1 -> m1 -> "Hello"
 *  - g1 references the variable it is assigned to (m1 here), and m1 owns the value of "Hello"
 *
 * referencing a value is called "borrowing" the value
 */
fn greet(g1: &String, g2: &String) {
    // note the ampersands
    println!("{} {}!", g1, g2);
}

/**
 * A dereference (marked with `*`) is a way to access to the original value of a pointer,
 * Since Boxes and references are pointers, you can use this to access to either the pointer
 */
fn dereference() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b = **r1; // two dereferences get us to the heap value
    let xx = 45;

    // mutable reference, we can use this to change the original value,
    // only works if the original pointer or value is mutable
    let mut r2: &mut i32 = &mut x; // r2 points to the heap value directly, which is the same as creating a Box
                                   // so the rules of ownership apply here
                                   // let r3 = r2;
    *r2 += 1; // <- compiler error, use of moved value
              // the think is since, we used a reference to access it,
              // it won't deallocate the memory when going out of scope
              // let c: i32 = *r2; // so only one dereference is needed to read it
    *x += 2;
    println!("Original value : {}", x);

    /**
     * Rust automatically uses dereferences when calling methods on references with the `.` operator,
     * Rust follows the pointer until it access the original value (even if the value needs to be accessed twice with `**var`)
     * Using `var.method()` is syntactic sugar for calling the function `method` with `var` as the first argument
     */
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice) to get the value
                                // since abs only takes the raw value not a reference
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);
}

fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num = &vec;
    // println!("Third element is {}", num[2]); // this works
    // We cannot mutate a variable while being borrowed (i.e referenced) If the reference is accessed later
    vec.push(4);
    // println!("Third element is {}", num[2]); // this does not work
}
