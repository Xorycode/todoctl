use std::path::PathBuf;
use std::fs::{read_to_string, File, OpenOptions};
use std::env::var_os;
use std::io;
use std::io::{Write, BufWriter};

fn add_item(todo_file_path: PathBuf) {
    // Get line to add
    print!("Enter item to add: ");
    io::stdout().flush().expect("Failed to flush STDOUT. May God have mercy on your soul, because this spells disaster.");
    let mut to_add = String::new();
    io::stdin().read_line(&mut to_add).expect("Failed to read STDIN. May God have mercy on your soul, because this spells disaster.");

    // Slight modifications
    to_add.insert_str(0, "- ");
    let to_add = to_add.trim();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(todo_file_path)
        .expect("Could not open todo.txt");

    writeln!(file, "{}", to_add).expect("Failed to write to todo.txt");
}

fn remove_item(todo_file_path: PathBuf) {
    let mut lines: Vec<String> = read_to_string(&todo_file_path)
        .expect("Could not read todo.txt")
        .lines()
        .map(String::from)
        .collect();
    
    println!("\nWhich item would you like to remove?");

    let mut counter: u32 = 1;
    for line in &lines {
        println!("{}: {}", counter, line);
        counter += 1;
    }
        
    let mut option = String::new();

    print!("\n> ");
    io::stdout().flush().expect("Failed to flush STDOUT. May God have mercy on your soul, because this spells disaster.");
    io::stdin().read_line(&mut option).expect("Failed to read STDIN. May God have mercy on your soul, because this spells disaster.");

    let option: u32 = option
        .trim()
        .parse::<u32>()
        .expect("Couldn't parse user's choice.") 
        - 1;

    lines.remove(option.try_into().expect("Could not convert u32 to usize. ???"));

    let file = File::create(todo_file_path).expect("Could not open todo.txt");
    let mut file_writer = BufWriter::new(file);
    
    for line in lines {
        writeln!(file_writer, "{}", line).expect("Could not write to todo.txt.");
        file_writer.flush().expect("Failed to flush changes to todo.txt");
    }
}


fn main() -> std::io::Result<()> {
    loop {
        let user_home = var_os("HOME").expect("Could not get $HOME");
        let mut todo_file_path = PathBuf::from(user_home);
        todo_file_path.push("todo");
        todo_file_path.set_extension("txt");
        if todo_file_path.exists() {
            let todo_contents = read_to_string(&todo_file_path).expect("Todo file found but cannot be opened.");
            println!("Current todo list is:");
            println!("{}", todo_contents);
        } else {
            File::create(&todo_file_path).expect("Creation failed.");
        }

        println!("
What would you like to do?
1) Add item to the todo list
2) Remove item from the todo list
3) Exit");


        let mut option = String::new();
        io::stdin().read_line(&mut option)?;
        let option: u32 = option.trim().parse().expect("Failed to parse");

        #[warn(clippy::main_recursion)]
        match option {
            1 => add_item(todo_file_path),
            2 => remove_item(todo_file_path),
            3 => std::process::exit(0),
            _ => main().expect("Failed to restart..?")
        }
    }
}
