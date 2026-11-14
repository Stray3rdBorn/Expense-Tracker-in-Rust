fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;
    let mut z = 100;

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let result = add(x, y);
    println!("Result of add(x, y): {result}");

    println!("Before reassignment: {z}");
    z = 200;
    println!("After z = 200: {z}");

    // String::from calls the from function associated with String.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 owns the moved string: {s2}");

    fn print_it(s: &String) {
        println!("Read-only borrow sees: {s}");
    }

    fn replace(text: &mut String) {
        *text = String::from("something else");
    }

    let mut s = String::from("hi");
    print_it(&s);
    replace(&mut s);
    println!("After the mutable borrow: {s}");
}

