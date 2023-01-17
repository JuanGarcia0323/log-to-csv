use regex::Regex;
use std::{
    fs::{self, create_dir_all, File},
    io::{ErrorKind, Write},
};

fn main() {
    let formated_files_path = String::from("./formated-files/");
    let files_path = String::from("./files-to-convert");
    create_dir_all(&formated_files_path).unwrap();
    let re = Regex::new(r"API").unwrap();
    let finded_files = fs::read_dir(&files_path).unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            create_dir_all(&files_path).unwrap();
            panic!("Insert log files into the files-to-convert folder");
        }
        panic!("{}", err)
    });

    return for file in finded_files {
        let f = file.unwrap();
        let file_name = f.file_name().to_str().unwrap().replace(".log", ".csv");
        println!("{}", &file_name);
        let new_path = format!("{}{}", formated_files_path, file_name);
        let path = f.path();
        let mut file = File::create(&new_path).unwrap();
        let content = fs::read_to_string(path).unwrap();
        let lines = content.lines();
        let mut first = false;
        for l in lines {
            if re.is_match(l) {
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
