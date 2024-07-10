use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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

    // Create the file
    let file_path = format!("{}/{}.md", dir, filename);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("Failed to create file");

    // Create unique identifier and links section
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M").to_string();

    // Format the file
    writeln!(file, "# \n\n\nLinks:\n{}", timestamp).expect("Failed to write to file");

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

    let second_brain = env::var("SECOND_BRAIN").expect("SECOND_BRAIN environment variable not set");
    let dir = format!("{}/0 Inbox", second_brain);

    open_file(&dir, &filename);
}
