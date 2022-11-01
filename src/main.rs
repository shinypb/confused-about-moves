use std::thread::spawn;

fn main() {
    println!("Hello, world!");
    let a_string: &str = "hotdogs";
    worker(&a_string);
}

fn worker(a_string: &str) {
    // It feels like this should work, but it doesn't
    // let a_string = a_string.clone();
    // But this does:
    let a_string = String::from(a_string);

    spawn(move || {
        println!("Touching the string {}", a_string);
    });
}
