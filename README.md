# Expense Tracker

A small command line expense tracker I built in Rust. I type expenses one per line, they are saved to a JSON file, and the program prints every saved expense with a total. I made it to learn Rust fundamentals on a real problem: modules, enums, parsing, error handling, and saving data to disk.

## Features

- Adds expenses from a single line like `12.50,Food,lunch`
- Five categories: Food, Rent, Bills, Clothes, Transport (case does not matter)
- Optional note, and the note may contain commas
- Saves everything to `expense.json` and loads it back on the next run
- Prints every saved expense and the total
- Bad lines are skipped with a clear message instead of stopping the program
- One unit test for the parser

## Requirements

Rust 1.85 or newer.

## Installation

```
git clone https://github.com/Stray3rdBorn/expense-tracker.git
cd expense-tracker
cargo build --release
```

## Usage

```
cargo run
```

Type one expense per line in this format, then press Enter on a blank line to finish:

```
amount,category,note
```

| Part       | Meaning                                 | Example                    |
| ---------- | --------------------------------------- | -------------------------- |
| `amount`   | A number, decimals allowed              | `12.50`                    |
| `category` | Food, Rent, Bills, Clothes or Transport | `food`                     |
| `note`     | Optional text, may contain commas       | `lunch with Sam, downtown` |

Run the tests with:

```
cargo test
```

## Example

Input:

```
12.50,Food,lunch
900,Rent
abc,Food
5,Games,x
3.2,Transport,bus, downtown
(blank line)
```

Output:

```
Skipping your input: amount must be a number
Skipping your input: Unknown category 'Games'. Use Food, Rent, Bills, Clothes, or Transport.
12.50 Food: lunch
900.00 Rent: (no note)
3.20 Transport: bus, downtown
Total: 915.70
```

On the next run the saved expenses load first, so new ones are added to the old list and the total covers all of them.

## Security and robustness

- Bad input never crashes the tool. A wrong amount, an unknown category or a missing part is skipped with a message.
- The amount must be a finite number, so `nan` and `inf` are rejected and the total can never become `NaN`.
- Negative amounts are allowed on purpose, so refunds and corrections work.
- Input is trimmed and the category is matched against a fixed list, so nothing else can be stored as a category.
- Every expense is saved to `expense.json` as soon as it is added, so closing the program early loses nothing.
- A missing `expense.json` is treated as a first run and starts an empty list.
- The note is stored as plain text and printed as text, never run as anything.
- No unsafe code and no network access.

Known limits:

- A corrupted `expense.json` stops the program with an error instead of recovering.
- The whole file is rewritten after each expense, so a crash in the middle of a write could corrupt it. Writing to a temporary file and renaming it would fix this.
- The data file is plain text in the folder where the program runs.
- Amounts are stored as decimal numbers, which can drift by tiny fractions, so not suitable for real accounting.
