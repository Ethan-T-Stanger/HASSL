use std::{env::args, fs::File, io::Read, path::PathBuf};

fn main() {
    let file_path = match get_file_path() {
        Option::None => return,
        Option::Some(file_path) => file_path,
    };

    let mut file = match File::open(&file_path) {
        Err(why) => panic!("Failed to open file {}: {}", file_path.display(), why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    let _file_length = match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Failed to read file {}: {}", file_path.display(), why),
        Ok(file_length) => file_length,
    };

    println!("{}", file_contents)
}

fn get_file_path() -> Option<PathBuf> {
    let mut path_buf = PathBuf::new();
    for arg in args() {
        if !arg.ends_with(".hassl") {
            continue;
        }

        path_buf.push(arg);
        return Option::Some(path_buf);
    }

    Option::None
}
