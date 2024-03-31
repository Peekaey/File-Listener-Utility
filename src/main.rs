mod commands;

use std::{fs, io, time};
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;
use notify::{Watcher, RecursiveMode, PollWatcher, Event, event};



fn main() {
    let mut file_exists = false;
    let mut original_file_path = String::new();
    
    let mut path_exists = false;
    let mut backup_save_path = String::new();
    
    while file_exists == false {
        println!("Please enter the full filepath of the file you wish to listen");
        // Using .clear() over String::new() to avoid allocating new memory = better performance
        original_file_path.clear();
        io::stdin().read_line(&mut original_file_path).expect("Failed to read line");
        
        original_file_path = original_file_path.trim().to_string();
        
        if fs::metadata(&original_file_path).is_ok() {
            println!("File exists");
            file_exists = true;
        } else {
            println!("File does not exist. Please enter a valid file path");
            println!("Current Value of Filepath: {}", original_file_path );
        }
    }
    
    
    while path_exists == false {
        println!("Please enter the full directory where you wish to save copies of the file");
        // Using .clear() over String::new() to avoid allocating new memory = better performance
        backup_save_path.clear();
        io::stdin().read_line(&mut backup_save_path).expect("Failed to read line");

        backup_save_path = backup_save_path.trim().to_string();
        
        if fs::metadata(&backup_save_path).is_ok() {
            println!("Path exists");
            path_exists = true;
        } else {
            println!("Path does not exist. Please enter a valid path");
            println!("Current Value of Path: {}", backup_save_path );
        }
    }
    
    // Manipulating the file name
    let save_path = commands::create_file_name(&original_file_path, backup_save_path);
    
    //Saving File
    commands::copy_file( &original_file_path, &save_path);
    
    
    
    
    //Creating a watcher to listen for changes
    let watcher_path = original_file_path.clone();
    println!("Watching Path: {}", watcher_path);
    
    if let Err(e) = commands::watch_poller(watcher_path) {
        println!("watch error: {:?}", e);
    }
    

    
}
