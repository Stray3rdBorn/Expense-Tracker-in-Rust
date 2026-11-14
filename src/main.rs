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

    // struct
    struct Expense {
        amount: f64,
        category: String,
        note: String,
    }
    let groceries = Expense {
        amount: 1000.0,
        category: String::from("food"),
        note: String::from("groceries"),
    };

    let coffee = Expense {
        amount: 5.0,
        category: String::from("food"),
        note: String::from("coffee"),
    };

    println!(
        "Groceries: {}, {}, {}",
        groceries.amount, groceries.category, groceries.note
    );
    println!(
        "Coffee: {}, {}, {}",
        coffee.amount, coffee.category, coffee.note
    );

    fn read_input() -> String {
        let mut value = String::new();
        std::io::stdin()
            .read_line(&mut value)
            .expect("Could not read input");
        value.trim().to_string()
    }

    let amount: f64 = read_input().parse().expect("Could not read input");
    let category = read_input();
    let note = read_input();
    let drinks: Expense = Expense {
        amount,
        category,
        note,
    };

    println!(
        "Drinks: {}, {}, {}",
        drinks.amount, drinks.category, drinks.note
    );
}
