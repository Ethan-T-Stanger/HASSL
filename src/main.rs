use std::{collections::HashMap, env::args, fs::File, io::Read, path::PathBuf};

enum ExitCode {
    Success,
    UnexpectedToken,
    EndOfFile,
}

fn terminate(_: &mut usize) -> Option<ExitCode> {
    Option::Some(ExitCode::Success)
}

fn advance(file_index: &mut usize) -> Option<ExitCode> {
    *file_index += 1;
    Option::None
}

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
    let file_contents = file_contents.chars().collect::<Vec<char>>();

    let mut file_index = 0;
    let commands: HashMap<char, fn(&mut usize) -> Option<ExitCode>> = HashMap::from([
        ('@', terminate as fn(&mut usize) -> Option<ExitCode>),
        (' ', advance),
    ]);

    let exit_code = loop {
        if file_contents.len() < file_index + 1 {
            break ExitCode::EndOfFile;
        }

        let function = match commands.get(&file_contents[file_index]) {
            None => break ExitCode::UnexpectedToken,
            Some(function) => function,
        };

        match function(&mut file_index) {
            None => (),
            Some(exit_code) => break exit_code,
        }
    };

    match exit_code {
        ExitCode::Success => return,
        ExitCode::UnexpectedToken => println!("Whops!: read an unexpected token."),
        ExitCode::EndOfFile => println!("Whops!: reached the end of the file."),
    }
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
