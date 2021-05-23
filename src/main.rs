// This program is intended for educational purposes only and not for securing files.
// I wrote this as an opportuniy to learn more about Rust and encryptions.
// Rust-Encryption lets you select an encryption from a list and then collects
// a string of text and encrypts it to show the before and after.

// Openssl for encryption methods
use openssl;
// Hex for encryption input
use hex;
// Write is for the flush method that allows input to be taken inline with print statement.
use std::io::{self, Write};

// Displays list of encryption options
fn available_encryptions(){
    print!("\n");
    let encryption_types = vec![
    "1: base64",
    "2: sha"
    ];

    println!("Available Encryptions:");
    for x in &encryption_types {
    println!("{}", x);
    }
    print!("\n");
}

// Collect number to choose encryption and make sure it is an integer.
fn select_number() -> i32 {
    loop {
        let mut number_selected = String::new();
        available_encryptions();
        print!("Select encryption type by number: ");
        let _ = io::stdout().flush(); //Flush makes the input be inline with the print statement
        std::io::stdin().read_line(&mut number_selected).unwrap();
        let trimmed = number_selected.trim();

        match trimmed.parse::<i32>() {
            Ok(number) => return number,
            Err(..) => println!("This was not an integer: {}", trimmed),
        };
    }
}

// Collect user input for string to be encrypted
fn collect_string() -> String {
    print!("Enter text to encrypt: ");
    let _ = io::stdout().flush(); //Flush makes the input be inline with the print statement
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).unwrap();
    println!("\n");

    return data;
}

// Matches your input to select the encryption type
fn encrypt(number: i32, text: &String) {
    println!("Your entry was: {}", text);
    match number {
        1 => base64(text),
        2 => sha(text),
        _ => println!("Not an option, select option from list. ")
    }
}

// base64 encryption
fn base64(text: &String) {
    let p = text.as_bytes();
    println!("Your entry encoded with base64 is: {}\n", openssl::base64::encode_block(p));
}

// The sha256 hash function
fn sha(text: &String) {
    let p = text.as_bytes();
    let v = openssl::sha::sha256(p);
    println!("{}\n", hex::encode(v));
}

fn main() {

    loop {

        // Gets encryption type and text and sends it to encryption to display
        let number = select_number();
        let text = collect_string();
        encrypt(number, &text);

        // Looks for "exit" and jumps out of the loop
        let mut check = String::new();
        print!("Restart / Exit: ");
        let _ = io::stdout().flush();
        std::io::stdin().read_line(&mut check).unwrap();
        if check.trim().to_lowercase() == "exit" {
            break;
        } else {
            print!("\n");
        }
    }
    return;

}
