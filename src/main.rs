mod commands;

use std::{fs, io};
use notify::{Watcher};

fn main() {
    let mut file_exists = false;
    let mut original_file_path = String::new();
    
    let mut path_exists = false;
    let mut backup_save_path = String::new();
    
    let mut polling_rate_valid = false;
    let mut string_polling_rate = String::new();
    let mut int_polling_rate = 0;
    
    while file_exists == false {
        println!("Please enter the full filepath of the file you wish to listen");
        // Using .clear() over String::new() to avoid allocating new memory = better performance
        original_file_path.clear();
        io::stdin().read_line(&mut original_file_path).expect("Failed to read line");
        
        original_file_path = original_file_path.trim().to_string();
        
        if fs::metadata(&original_file_path).is_ok() {
            println!("File existence validated");
            file_exists = true;
        } else {
            println!("File not found. Please enter a valid file path");
        }
    }
    
    while path_exists == false {
        println!("Please enter the full directory where you wish to save copies of the file");
        // Using .clear() over String::new() to avoid allocating new memory = better performance
        backup_save_path.clear();
        io::stdin().read_line(&mut backup_save_path).expect("Failed to read line");

        backup_save_path = backup_save_path.trim().to_string();
        
        if fs::metadata(&backup_save_path).is_ok() {
            println!("Path existence validated");
            path_exists = true;
        } else {
            println!("Path does not exist. Please enter a valid path");
        }
    }
    
    while polling_rate_valid == false {
        println!("Please enter the polling rate in seconds (Recommended 60)");
        string_polling_rate.clear();
        io::stdin().read_line(&mut  string_polling_rate).expect("Failed to read line");
        
        int_polling_rate = match string_polling_rate.trim().parse() {
            Ok(num) => {
                println!("Valid polling Rate");
                polling_rate_valid = true;
                num
            }
            Err(e) => {
                println!("Invalid input, Please enter a valid number");
                continue;
            }
        };
    }
    
    // Creating backup filename and path
    let save_path = commands::create_file_name(&original_file_path, &backup_save_path);
    
    //Saving File
    commands::copy_file( &original_file_path, &save_path);
    
    //Handling off initial values
    let string_save_path = backup_save_path.clone();
    let string_file_path = original_file_path.clone();
    
    //Creating watcher path to listen for changes
    let watcher_path = original_file_path.clone();
    println!("Watching Path: {}", watcher_path);
    
    if let Err(e) = commands::watch_poller(watcher_path, string_save_path, string_file_path, int_polling_rate) {
        println!("watch error: {:?}", e);
    }
}
