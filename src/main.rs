fn scope_string() {
    // s is not valid here, since it's notyet declared
    let _s = "hello"; // s is valid from this point forward
                      // do stuff with s
} // this scope is now over, and s is nolonger valid

fn mutate_string() {
    let mut s = String::from("hello"); // s is valid from this point forward
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `heelo, world!`
} // this scope is now over, and s is no
  // longer valid

fn variables_data_interacting_w_move() {
    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); // -> won't work because we use it after move

    let s3 = s2.clone();

    println!("s2 = {s2}, s3 = {s3}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called.
  // The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes
                                             // into scope
    some_string // some_string is returned and moves out to the
                // calling function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes
    // into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But becuase it does not have ownership of what
  // it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped, so its memory goes away
// the solution is to return the String directly

fn main() {
    scope_string();
    mutate_string();
    variables_data_interacting_w_move();

    let s = String::from("HELLO"); // s comes into scope
    takes_ownership(s); // s's value moves into the function ...
                        // ... and so is no longer valid here
                        // println!("{s}"); // <- won't work because
                        // value was borrowed
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{x}"); // <- works because value i32 is Copy

    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back
                                        // which also moves its return value into s3
    let s4 = String::from("yolo");
    let (s5, len) = calculate_length(s4);

    println!("The length of '{s5}' is {len}.");

    let sref = String::from("hello");
    let len = calculate_length_ref(&sref);

    println!("The length of '{sref}' is {len}.");

    let mut to_be_changed = String::from("hellow");

    change(&mut to_be_changed);
    println!("Mutable references: {to_be_changed}");

    let mut to_be_borrowed = String::from("hello");
    // let r1 = &mut to_be_borrowed; // first mutable borrow occurs here
    // let r2 = &mut to_be_borrowed; // second mutable borrow occurs here
    // The first mut borrow is in r1 and must last until it's used in the
    // println!
    // println!("{r1}, {r2}"); <- this generates the error
    {
        let _r1 = &mut to_be_borrowed;
    } // r1 goes out of scope here, so we can make a new reference with
      // no problems
    let _r2 = &mut to_be_borrowed;
    // Rust enforces a similar rule for combining mutable and immutable
    // references
    let _r3 = &to_be_borrowed;
    // let _r4 = &mut to_be_borrowed; // <- cannot borrow as mutable because
    // it is also borrowed as immutable
    // println!("{_r3}, {_r4}");
    let mut s_scope = String::from("SCOPE");

    let r1_scope = &s_scope; // no problem
    let r2_scope = &s_scope; // no problem

    println!("{r1_scope} and {r2_scope}");
    // variables r1_scope and r2_scope will not be used after this point

    let r3 = &mut s_scope; // no problem
    println!("{r3}");

    // let reference_to_nothing = dangle(); // expected named lifetime param
    // this function's return type contains a borrowed value, but there is
    // no value for it to be borrowed from
} // Here, x goes out of scope, then s. However, because s's
  // value was moved, nothing special happens
  // Here, s3 goes out of scope and is dropped. s2 was moved,
  // so nothing happens. s1 goes out of scope and is dropped.
