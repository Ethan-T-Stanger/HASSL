mod commands;
mod types;

use commands::{
    add, advance, continue_if, continue_not, count, generate_random, go_to_state, increment,
    line_input, numeric_print, pop, print, push, reset, select_left, select_right, set_state,
    subtract, terminate,
};
use regex::Regex;
use std::{collections::HashMap, env::args, fs::File, io::Read, path::PathBuf};
use types::{Direction, ExitCode, ProgramData, State};

fn main() {
    let file_path = match get_file_path() {
        Option::None => return,
        Option::Some(file_path) => file_path,
    };

    let mut file = match File::open(&file_path) {
        Err(why) => panic!("Failed to open {}: {}", file_path.display(), why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Failed to read {}: {}", file_path.display(), why),
        Ok(_) => (),
    };
    let regex = Regex::new(r"(%.*%)").unwrap();

    let file_contents = String::from(regex.replace_all(&file_contents, ""))
        .chars()
        .filter(|char| *char != ' ' && *char != '\t' && *char != '\n' && *char != ':')
        .collect::<Vec<char>>();

    if file_contents.iter().any(|char| *char == '%') {
        println!("hassl-err!: found an unclosed comment.");
        return;
    }

    let commands = HashMap::from([
        (
            '@',
            terminate as fn(&mut ProgramData, &Vec<char>) -> Option<ExitCode>,
        ),
        (' ', advance),
        ('&', go_to_state),
        ('?', continue_if),
        ('!', continue_not),
        ('$', set_state),
        ('^', push),
        ('v', pop),
        ('g', line_input),
        ('<', select_left),
        ('>', select_right),
        ('*', increment),
        ('.', reset),
        ('+', add),
        ('-', subtract),
        ('#', count),
        ('p', print),
        ('n', numeric_print),
        ('~', generate_random),
        ('0', advance),
        ('1', advance),
        ('2', advance),
        ('3', advance),
        ('4', advance),
        ('5', advance),
        ('6', advance),
        ('7', advance),
        ('8', advance),
        ('9', advance),
        ('A', advance),
        ('B', advance),
        ('C', advance),
        ('D', advance),
        ('E', advance),
        ('F', advance),
    ]);

    let mut program_data = ProgramData {
        file_index: 0,
        direction: Direction::Right,
        left_stack: Vec::new(),
        right_stack: Vec::new(),
        register_value: 0,
        selected_state: State::State0,
    };

    let exit_code = go_to_state(&mut program_data, &file_contents);

    let exit_code = if exit_code.is_some() {
        exit_code.unwrap()
    } else {
        loop {
            if file_contents.len() < program_data.file_index + 1 {
                break ExitCode::EndOfFile;
            }

            let function = match commands.get(&file_contents[program_data.file_index]) {
                None => break ExitCode::TokenUndefined,
                Some(function) => function,
            };

            match function(&mut program_data, &file_contents) {
                None => (),
                Some(exit_code) => break exit_code,
            }
        }
    };

    print_exit_code_message(exit_code);
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

fn print_exit_code_message(exit_code: ExitCode) {
    let prefix = "hassl-err!:";
    match exit_code {
        ExitCode::Success => return,
        ExitCode::EndOfFile => eprintln!("{} end of file", prefix),
        ExitCode::TokenUndefined => eprintln!("{} token undefined", prefix),
        ExitCode::StateUndefined => eprintln!("{} state undefined", prefix),
        ExitCode::StackUnderflow => eprintln!("{} stack underflow", prefix),
        ExitCode::InternalError => eprintln!("{} internal error occurred", prefix),
    }
}
