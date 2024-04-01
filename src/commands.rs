use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use chrono;
use chrono::Local;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{notify::*};

pub fn copy_file(original_file_path: &String, save_path: &String) -> bool {
    return if fs::copy(&original_file_path, &save_path).is_ok() {
        println!("File copied successfully, saved to: {}", save_path);
        true
    } else {
        println!("Failed to copy file");
        false
    }
}

pub fn create_file_name(file_path: &String, backup_save_path: &String) -> String {
    let full_file_name = file_path.split("/").last().unwrap();
    let file_name_parts: Vec<_> = full_file_name.split(".").collect();
    let file_name = file_name_parts[0];
    let file_extension = file_name_parts[1];
    let new_file_name = format!("{} {} {} {}", file_name, Local::now().format("%d-%m-%Y"), Local::now().format("%H-%M-%S"), file_extension);
    let new_backup_save_path = backup_save_path.to_owned() + "/" + new_file_name.as_str();
    println!("New Save Path: {}", new_backup_save_path);
    return new_backup_save_path;
}

// Automatic Polling with updating next polling time
pub fn watch_poller<P: AsRef<Path>>(path: P, string_save_path: String, string_file_path: String, polling_rate: u64) -> notify::Result<()> {
    let (tx, rx) = channel();
    // use the PollWatcher and disable automatic polling
    let mut watcher = PollWatcher::new(tx, notify::Config::default().with_manual_polling())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    // run event receiver on a different thread
    std::thread::spawn(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    let temp_save_path = string_save_path.clone();
                    if let new_string_save_path = create_file_name(&string_file_path, &temp_save_path) {
                        if copy_file(&string_file_path, &new_string_save_path) {
                            println!("File Event Change Type: {:?}", event);
                        } else {
                            println!("File had no changes");
                        }
                    } 
                }
                Err(e) => {
                    println!("Error while watching file: {:?}", e)
                }
            }
        }
    });

    // Set up a timer to trigger polling every 60 seconds
    let mut last_poll_time = Instant::now();
    let poll_interval = Duration::from_secs(polling_rate);

    loop {
        let elapsed = last_poll_time.elapsed();
        if elapsed >= poll_interval {
            println!("Polling for changes...");
            watcher.poll().unwrap();
            last_poll_time = Instant::now();
        } else {
            let remaining_time = poll_interval - elapsed;
            print!("Next poll in {} seconds", remaining_time.as_secs());
            // Flush the print buffer to ensure the message is displayed immediately
            let _ = std::io::stdout().flush();
            // Use carriage return to overwrite the previous line
            print!("\r");
            // Sleep for a short duration to prevent excessive CPU usage
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

// Below functions are just examples not currently in use
// Implemented as per https://github.com/notify-rs/notify/blob/18df78efff79a6eb7e1305280957c4b3ae43e1df/examples/monitor_raw.rs
pub fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Default::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

//Implemented as per https://github.com/notify-rs/notify/blob/main/examples/pollwatcher_manual.rs
pub  fn manual_watch_poller<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    // use the PollWatcher and disable automatic polling
    let mut watcher = PollWatcher::new(tx, Config::default().with_manual_polling())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    // run event receiver on a different thread, we want this one for user input
    std::thread::spawn(move || {
        for res in rx {
            match res {
                Ok(event) => println!("changed: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    // wait for any input and poll
    loop {
        println!("Press enter to poll for changes");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        println!("polling..");
        // manually poll for changes, received by the spawned thread
        watcher.poll().unwrap();
    }
}