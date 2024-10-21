use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;
use chrono::Local;

fn get_filename() -> String {
    print!("Enter a filename: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    filename.trim().to_string()
}

fn open_file(dir: &str, filename: &str) {
    // Change directory
    env::set_current_dir(dir).expect("Failed to change directory");

    // Create the file path
    let file_path = format!("{}/{}.md", dir, filename);

    // Create file and write Jekyll front matter
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("Failed to create file");

    // Get current date and time for the front matter
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string(); 
    let time = now.format("%H:%M:%S %z").to_string(); 

    // Write Jekyll front matter
    writeln!(file, "---").expect("Failed to write to file");
    writeln!(file, "layout: post").expect("Failed to write to file");
    writeln!(file, "title:  \"{}\"", filename).expect("Failed to write to file");
    writeln!(file, "date:   {} {}", date, time).expect("Failed to write to file"); 
    writeln!(file, "categories:").expect("Failed to write to file");
    writeln!(file, "tags:").expect("Failed to write to file");
    writeln!(file, "---").expect("Failed to write to file");  

    // Add content after front matter (optional)
    writeln!(file, "\n# {}\n\n", filename).expect("Failed to write to file");  

    // Open the file in Neovim
    Command::new("nvim")
        .arg("+ normal ggzzi")
        .arg(&file_path)
        .arg("-c")
        .arg(":NoNeckPain")
        .status()
        .expect("Failed to open file in Neovim");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() == 1 {
        get_filename()
    } else if args.len() > 2 {
        eprintln!("Please provide only one filename separated by dashes, without .md extension.");
        eprintln!("Example: zet my-new-note");
        std::process::exit(1);
    } else {
        args[1].clone()
    };

    let second_brain = env::var("SECOND_BRAIN").unwrap_or_else(|_| {
        eprintln!("Error: SECOND_BRAIN environment variable not set.");
        std::process::exit(1);
    });

    let dir = format!("{}/1 Summary Notes", second_brain);

    open_file(&dir, &filename);
}
