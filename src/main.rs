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

fn main() {
    scope_string();
    mutate_string();
    variables_data_interacting_w_move();
}
