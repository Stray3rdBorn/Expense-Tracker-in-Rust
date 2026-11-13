fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;
    let mut z = 100;

    fn add(x: i32, y: i32) -> i32 { 
        x + y 
    }

    let result = add(x, y);
    println!("Result: {result} ")
}

