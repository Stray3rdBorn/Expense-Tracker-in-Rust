// the expense-related types and functions grouped under the `expense` name.
mod expense {
    // Serialize means “turn this into data to save”; Deserialize means “rebuild it from saved data.”
    use serde::{Deserialize, Serialize};
    // print categories for debugging and compare them in tests.
    #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
    // five choices.
    pub enum ExpenseCategory {
        Food,
        Rent,
        Bills,
        Clothes,
        Transport,
    }

    // information stored for one expense.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Expense {
        //  the expense amount as a decimal number.
        pub amount: f64,
        //  the expense's category.
        pub category: ExpenseCategory,
        //  an optional note: Some(text) if present, or None if absent.
        pub note: Option<String>,
    }

    // Convert one comma-separated text line into an Expense, or return an error message.
    pub fn parse_expense(line: &str) -> Result<Expense, String> {
        // Split the line at commas, with at most three pieces so the note may contain commas.
        let mut parts = line.splitn(3, ',');

        // take the first piece as the amount or report that it was missing.
        let amount_text = parts.next().ok_or_else(|| "missing amount".to_string())?;
        // take the second piece as the category or report that it was missing.
        let category_text = parts.next().ok_or_else(|| "missing category".to_string())?;

        // remove surrounding spaces, then convert the amount text into an f64 number.
        let amount = amount_text
            .trim()
            // return a readable error instead if the text is not a valid number.
            .parse::<f64>()
            .map_err(|_| "amount must be a number".to_string())?;

        // match the category text to a known variant
        let category = match category_text.trim().to_lowercase().as_str() {
            "food" => ExpenseCategory::Food,
            "rent" => ExpenseCategory::Rent,
            "bills" => ExpenseCategory::Bills,
            "clothes" => ExpenseCategory::Clothes,
            "transport" => ExpenseCategory::Transport,
            _ => {
                return Err(format!(
                    "Unknown category '{category_text}'. Use Food, Rent, Bills, Clothes, or Transport."
                ));
            }
        };

        // take the third piece as the note, use empty text if no third piece exists.
        let note_text = parts.next().unwrap_or("").trim();
        // represent a blank note as None and a non-blank note as Some(String).
        let note = if note_text.is_empty() {
            // there is no note to store.
            None
        } else {
            // make an owned String so the note can be kept in the Expense.
            Some(note_text.to_string())
        };

        // build an Expense from the parsed values and return it as a successful result.
        Ok(Expense {
            amount,

            category,

            note,
        })
    }

    // print one expense without taking ownership of it.
    pub fn print_expense(expense: &Expense) {
        // borrow the note text, or display a placeholder when the note is absent.
        let note = expense.note.as_deref().unwrap_or("(no note)");
        // print the amount to two decimal places, then the category and note.
        println!("{:.2} {:?}: {}", expense.amount, expense.category, note);
    }

    // test module only when running tests.
    #[cfg(test)]
    // keep tests for the expense module next to the code.
    mod tests {
        // bring names from the surrounding expense module into this test module.
        use super::*;

        // function as an automated test for `cargo test`.
        #[test]
        // check if expense line is parsed correctly.
        fn parses_a_valid_expense() {
            // parse one sample line, unwrap is fine here because the test expects success.
            let expense = parse_expense("12.50,Food,lunch").unwrap();

            assert_eq!(expense.amount, 12.50);

            assert_eq!(expense.category, ExpenseCategory::Food);

            assert_eq!(expense.note.as_deref(), Some("lunch"));
        }
    }
}

use std::{fs::read, path::Prefix::VerbatimDisk};

// expense module's names available to the main function.
use expense::{ExpenseCategory, parse_expense, print_expense};

// main rust program
fn main() {
    // let creates expenses, mut means the list can change, Vec means a growable list, match checks whether reading the file worked
    let mut expenses: Vec<expense::Expense> = match std::fs::read_to_string("expense.json") {
        // Ok means the file was read, json holds its text, from_str turns that text into expenses, expect stops with a message if the JSON is invalid
        Ok(json) => serde_json::from_str(&json).expect("expense.json contains invalid JSON"),
        // Err means reading failed, if notfound checks whether the file simply doesn't exist, if so start an empty list
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        // handles other file errors by stopping and showing the error
        Err(error) => panic!("Could not read expenses.json: {error}"),
    };

    loop {
        let line = read_input();

        if line.is_empty() {
            break;
        }
        println!("You entered: {line}");
        match parse_expense(&line) {
            Ok(expense) => expenses.push(expense),
            Err(error) => println!("Skipping your input: {error}"),
        }
    }

    // take input from terminal
    fn read_input() -> String {
        let mut input = String::new();
        println!(
            "Enter amount, category, note (example: 12.50,Food,lunch). Categories: Food, Rent, Bills, Clothes, Transport. Press Enter on a blank line to finish."
        );
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");
        input.trim().to_string()
    }

    // turn the expense list into readable json
    let json = serde_json::to_string_pretty(&expenses).expect("Could not convert expenses to JSON");
    // write it to expense.json
    std::fs::write("expense.json", json).expect("Could not save expenses to expense.json");
    // borrow each saved expense in turn so it can be printed without removing it from the list.
    for expense in &expenses {
        print_expense(expense);
    }

    // read each amount, transform the amounts into a sequence, and add them together.
    let total: f64 = expenses.iter().map(|expense| expense.amount).sum();
    println!("Total: {total:.2}");
}
