#![allow(unused)]
// Constants can be declared in the global scope, and variables can't
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn vars() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let str = "am i not a scalar ?";
    let bin = 0b1111_0000u8;
    let byte = b'a';
    println!("Binary {byte}");
    // you can add suffix at the end of numbers to indicate their type
    let mut u = 2u8;

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
                      // Casting is done with `as` like in typescript
    let x = y as f64 / 21f64;
    let t = (x as f32) == y;

    // Chars occupy up to 4 bytes, which make it possible to use utf8 chars (emoji, chinese, korean, russian, etc.)
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let heart_eyed_cat_num = heart_eyed_cat.len_utf8();

    println!("Floating {heart_eyed_cat} {heart_eyed_cat_num} {c} {z}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;

    // Swap 2 values
    let a = 5;
    let b = 6;
    let (a, b) = (b, a);

    println!("Swapping vars : {a}, {b}");
    // a tuple without a value `()` is called `unit`
    let unit = ();

    // you declare an array type with `[type; length]`
    let mut arr: [i32; 5] = [0; 5]; // initialization with all 0 as values
    arr[heart_eyed_cat.len_utf8()] = 14;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let message = "The temperature today is:";

    // you can initialize an array filled with a variable value
    let x = [message; 100];

    println!("{} {}", x[0], x[1]);

    // You can assign arrays to tuples
    let t = ([1; 2], [3; 4]);

    let (a, _) = t;

    println!("{}", a[0] + t.1[0]);
}

fn another_function(x: i32) {
    // A block is an expression, the last value in the block can be assigned to a variable
    let y = {
        let x = 3;
        x + 1 //; if you add a semicolon at the end of the expression, the value returned by the block is a unit `()`,
              // it is turned into a statement
    };

    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5 // you can return an expression without using `return` keyword, just don't forget to not add a semicolon
}

// A function must provide a parameter & return type
// if the return type is not provided, the implicit return type is unit `()`
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn f(x: i32) -> i32 {
    x + 1
}

fn loops() {
    let number = 6;

    // Conditions don't need parenthesis
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // to implement ternaries you use an inline condition, that returns an expression
    // all the arms of the expression must return the same type
    // an else is always mandatory
    let number = if condition {
        let x = 5;
        x + 1
    } else {
        12
    };

    println!("The value of number is: {number}");

    // infinite loop with `loop`
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // you can return a value from a loop with the syntax : `break <return-value>`
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    // a loop can have a label (it must start with a single quote and colon)
    // Only loops & blocks can have a label
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // you can break a loop by its label
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops works normally
    // You can't return a value with break in a while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let collection = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < collection.len() {
        println!("the value is: {}", collection[index]);

        index += 1;
    }

    // you can use a `for` loop for iterating over a collection of items
    for element in collection {
        println!("the value is: {element}");
    }

    // We can use a Range (a..b) to iterate a fixed number of times
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn facto(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }

    n * facto(n - 1)
}

fn fizzbuzz() {
    for x in 1..=100 {
        let mut output = String::new();

        if x % 3 == 0 {
            output += "fizz"
        }
        if x % 5 == 0 {
            output += "buzz"
        }

        println!("{x} => {output}")
    }
}

fn fibonnaci(n: u32) -> u128 {
    if n == 0 || n == 1 {
        return n as u128;
    }

    if n == 2 {
        return 1;
    }

    // fibonnaci(n - 1) + fibonnaci(n - 2)
    let mut previous: u128 = 0;
    let mut current: u128 = 1;
    for i in 3..=n {
        let next_num = previous + current;
        previous = current;
        current = next_num;
    }

    current
}
