use std::fs::{self};
use std::io::{self,  Write};
use std::path::{Path, PathBuf};

fn main() {
    let mut current_directory = PathBuf::from(".");
    loop{
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Please enter a command");
        let input = input.trim();
        let parts_of_input:Vec<&str> = input.split(' ').collect(); //returns vector of &str
        // let current_directory = current_directory.clone();
        match parts_of_input[0] {
            "cd"=>{
                let new_dir = Path::new(&current_directory).join(parts_of_input[1]);
                if !new_dir.exists(){
                    println!("ERRRRRRRRROR: {} directory does not exist",new_dir.display());
                }else if !new_dir.is_dir(){
                    println!("ERRRRRRRRROR: {} is not a directory",new_dir.display());
                }else{
                    current_directory = new_dir;
                }
            }
            "ls"=>{
                let items = fs::read_dir(&current_directory).unwrap();
                for item in items{
                    let item = item.unwrap();
                    let path = item.path();
                    let display = path.display();
                    let metadata = item.metadata().unwrap();
                    let filetype = metadata.file_type();
                    if filetype.is_dir(){
                        println!("{} [DIR]",display);
                    }else{
                        println!("{} [FILE]",display);
                    }
                }
            }
            "mkdir"=>{
                let new_dir = Path::new(&current_directory).join(parts_of_input[1]);
                if new_dir.exists(){
                    println!("ERRRROR: {} already exists",new_dir.display());
                }else{
                    fs::create_dir(new_dir).unwrap();
                }
            }
            "rm"=>{
                let directory_to_remove = Path::new(&current_directory).join(parts_of_input[1]);
                if !directory_to_remove.exists(){
                    println!("ERRRROR: {} doesnt exist",directory_to_remove.display());
                }else{
                    fs::remove_dir(directory_to_remove).unwrap();
                }
            }
            "quit"=> break,
            _=>println!("ERRRRROR: {} is not a valid command",parts_of_input[0])
        }
    }
}
