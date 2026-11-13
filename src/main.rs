fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;
    let mut z = 100;

    // function

    fn add(x: i32, y: i32) -> i32 { 
        x + y 
    }

    let result = add(x, y);
    println!("Result: {result} ");

        // ownership
    let s1 = String::from("hello");
    let s2 = s1;              
    println!("{}", s2);     

    // borrow
    fn print_it(s: &String) {
        println!("{}", s)

    }
    let s = String::from("hi");
    print_it(&s);
    println!("{}", s);
}

