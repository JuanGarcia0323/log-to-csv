use regex::Regex;
use std::{
    fs::{self, create_dir_all, File},
    io::{ErrorKind, Write},
};

fn main() {
    // Folders for the files
    let formated_files_path = String::from("./formated-files/");
    let files_path = String::from("./files-to-convert");
    // Create the folder if it not exists
    create_dir_all(&formated_files_path).unwrap();
    // Regular expression to look for the word API
    let re = Regex::new(r"API").unwrap();
    // Get the files and create the folder files_path if it not exist
    let finded_files = fs::read_dir(&files_path).unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            create_dir_all(&files_path).unwrap();
            panic!("Insert log files into the files-to-convert folder");
        }
        panic!("{}", err)
    });

    // Iterate over the files
    return for file in finded_files {
        let f = file.unwrap();
        // Create the file name with the correct format file
        let file_name = f.file_name().to_str().unwrap().replace(".log", ".csv");
        println!("{}", &file_name);
        let new_path = format!("{}{}", formated_files_path, file_name);
        let path = f.path();
        // Create the new file
        let mut file = File::create(&new_path).unwrap();
        // Read the content of the file that we are converting
        let content = fs::read_to_string(path).unwrap();
        let lines = content.lines();
        // Check if it's the first file
        let mut first = false;
        for l in lines {
            if re.is_match(l) {
                // Generate conditional for the first file and preparing information for the csv format
                let result = if !first {
                    String::from(l)
                        .replace(r#"""#, "")
                        .replace(" ", ",")
                        .replace(",", ", ")
                } else {
                    format!("\n{}", l)
                        .replace(r#"""#, "")
                        .replace(" ", ",")
                        .replace(",", ", ")
                };
                first = true;
                file.write(result.as_bytes()).unwrap();
            }
        }
    };
}
