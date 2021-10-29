use std::io;

fn main() {
    // Display program description
    println!("This program replace each character in a string with another character specified in a 26-character key, with each character in the key corresponding to an alphabet.");
    println!("The key should contain all 26 characters with no duplicate.");
    println!("Please enter the 26-character key, the first character in the key will replace A and the second character will replace B in your message, etc.");

    // Take user input for key, rejects everything not qualified as key
    let mut key = String::new();
    loop {
        key.clear();
        io::stdin()
            .read_line(& mut key)
            .expect("Not valid key!");
    
        let mut valid = true;
        key = key.to_uppercase().trim().to_string(); // Conver input to all uppercase for easier processing
        match key.len() { // Check key length
            26 => {
                for c in key.trim().chars() { // Check if the key contains non-alphabet characters
                    match c {
                        'A'..='Z' => (),
                        _ => valid = false
                    }
                }
            },
            _=> valid = false
        }

        match check_duplicate(&key) { // Check if the key contains duplicate characters
            true => {
                valid = false;
                println!("There are duplicate letters in the keys.")
            },
            false => ()
        }
    
        match valid {
            true => break,
            false => println!("Not valid Key!")
        }
    }

    // Create character set and keyset vector containing both uppercase and lowercase character with index
    let upper_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut char_index: Vec<char> = Vec::new();
    let mut key_index: Vec<char> = Vec::new();
    for c in upper_chars.chars() {
        char_index.push(c)
    }
    for c in upper_chars.to_lowercase().chars() {
        char_index.push(c)
    }
    for c in key.chars() {
        key_index.push(c)
    }
    for c in key.to_lowercase().chars() {
        key_index.push(c)
    }

    loop{    
        // Take user input for the message to be encrypted
        println!("Please enter the message you want to encrypt with the key:");
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Not valid text!");
    
        let mut encrypted = String::new(); // Create a new string to store encrypted message
    
        // When a character in the message to be encrypted matches one in the character index, push the corresponding character in the key index to the encrypted string,
        // otherwise push the origianl character into the encrypted message
        for c in message.chars() {
            let mut counter = 0;
            let mut replace = false;
            for char in &char_index {
                if c == *char {
                    replace = true;
                    encrypted.push(key_index[counter])
                }
                counter += 1
            }
            if replace == false {
                encrypted.push(c)
            }
        }
        print!("{}", encrypted);
    }
}

fn check_duplicate(key: &String) -> bool {
    let mut duplicate = false;

    // Compare each characters in the string against all characters in the string, increase counter by 1 if they match
    let mut counter = 0;
    for c in key.chars() {
        for char in key.chars() {
            if c == char {
                counter += 1
            }
        }
    }

    // Counter should be the same as the length of the key string if there's no duplicate characters
    if counter > key.len() {
        duplicate = true
    }

    return duplicate
}