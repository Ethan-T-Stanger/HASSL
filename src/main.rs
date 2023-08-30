use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

use rand::random;

enum ExitCode {
    Success,
    UnexpectedToken,
    EndOfFile,
    UnselectedDirection,
    StackUnderflow,
    Input,
}

enum Direction {
    Unselected,
    Left,
    Right,
}

struct State {
    file_index: usize,
    direction: Direction,
    left_stack: Vec<u8>,
    right_stack: Vec<u8>,
    register_value: u8,
}

fn terminate(_: &mut State) -> Option<ExitCode> {
    Option::Some(ExitCode::Success)
}

fn advance(state: &mut State) -> Option<ExitCode> {
    state.file_index += 1;
    Option::None
}

fn push(state: &mut State) -> Option<ExitCode> {
    let register_value = state.register_value;
    match get_stack(state) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => stack.push(register_value),
    }
    advance(state);
    Option::None
}

fn pop(state: &mut State) -> Option<ExitCode> {
    match get_stack_value(state) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(value) => state.register_value = value,
    };
    advance(state);
    Option::None
}

fn line_input(state: &mut State) -> Option<ExitCode> {
    let stack = match get_stack(state) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => stack,
    };
    let mut input_string = String::new();
    match stdin().read_line(&mut input_string) {
        Err(_) => return Option::Some(ExitCode::Input),
        Ok(_) => (),
    };
    input_string.chars().for_each(|char| {
        stack.push(match u8::try_from(char) {
            Err(_) => char::REPLACEMENT_CHARACTER as u8,
            Ok(value) => value,
        })
    });
    advance(state);
    Option::None
}

fn select_left(state: &mut State) -> Option<ExitCode> {
    state.direction = Direction::Left;
    advance(state);
    Option::None
}

fn select_right(state: &mut State) -> Option<ExitCode> {
    state.direction = Direction::Right;
    advance(state);
    Option::None
}

fn increment(state: &mut State) -> Option<ExitCode> {
    state.register_value = match state.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => state.register_value.wrapping_add(16),
        Direction::Right => state.register_value.wrapping_add(1),
    };
    advance(state);
    Option::None
}

fn reset(state: &mut State) -> Option<ExitCode> {
    state.register_value = match state.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => state.register_value % 16,
        Direction::Right => state.register_value / 16 * 16,
    };
    advance(state);
    Option::None
}

fn add(state: &mut State) -> Option<ExitCode> {
    match get_stack_values(state) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            state.register_value = first_value.wrapping_add(second_value)
        }
    };
    advance(state);
    Option::None
}

fn subtract(state: &mut State) -> Option<ExitCode> {
    match get_stack_values(state) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            state.register_value = first_value.wrapping_sub(second_value)
        }
    };
    advance(state);
    Option::None
}

fn count(state: &mut State) -> Option<ExitCode> {
    match get_stack(state) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => {
            state.register_value = match u8::try_from(stack.len()) {
                Err(_) => u8::MAX,
                Ok(value) => value,
            }
        }
    }
    advance(state);
    Option::None
}

fn print(state: &mut State) -> Option<ExitCode> {
    print!(
        "{}",
        if state.register_value.is_ascii() {
            state.register_value as char
        } else {
            char::REPLACEMENT_CHARACTER
        }
    );
    advance(state);
    Option::None
}

fn numeric_print(state: &mut State) -> Option<ExitCode> {
    print!("{}", state.register_value);
    advance(state);
    Option::None
}

fn generate_random(state: &mut State) -> Option<ExitCode> {
    state.register_value = random();
    advance(state);
    Option::None
}

fn get_stack(state: &mut State) -> Result<&mut Vec<u8>, ExitCode> {
    let stack = match state.direction {
        Direction::Unselected => return Err(ExitCode::UnselectedDirection),
        Direction::Left => &mut state.left_stack,
        Direction::Right => &mut state.right_stack,
    };
    Ok(stack)
}

fn get_stack_value(state: &mut State) -> Result<u8, ExitCode> {
    let stack = match get_stack(state) {
        Err(exit_code) => return Err(exit_code),
        Ok(stack) => stack,
    };
    match stack.pop() {
        None => return Err(ExitCode::StackUnderflow),
        Some(value) => Ok(value),
    }
}

fn get_stack_values(state: &mut State) -> Result<(u8, u8), ExitCode> {
    let stack = match get_stack(state) {
        Err(exit_code) => return Err(exit_code),
        Ok(stack) => stack,
    };
    let first_value = match stack.pop() {
        None => return Err(ExitCode::StackUnderflow),
        Some(value) => value,
    };
    let second_value = match stack.pop() {
        None => return Err(ExitCode::StackUnderflow),
        Some(value) => value,
    };
    Ok((first_value, second_value))
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
    let commands = HashMap::from([
        ('@', terminate as fn(&mut State) -> Option<ExitCode>),
        (' ', advance),
        ('^', push),
        ('v', pop),
        ('g', line_input),
        ('n', numeric_print),
        ('<', select_left),
        ('>', select_right),
        ('*', increment),
        ('.', reset),
        ('+', add),
        ('-', subtract),
        ('#', count),
        ('p', print),
        ('~', generate_random),
    ]);

    let mut state = State {
        file_index: 0,
        direction: Direction::Unselected,
        left_stack: Vec::new(),
        right_stack: Vec::new(),
        register_value: 0,
    };

    let exit_code = loop {
        if file_contents.len() < state.file_index + 1 {
            break ExitCode::EndOfFile;
        }

        let function = match commands.get(&file_contents[state.file_index]) {
            None => break ExitCode::UnexpectedToken,
            Some(function) => function,
        };

        match function(&mut state) {
            None => (),
            Some(exit_code) => break exit_code,
        }
    };

    match exit_code {
        ExitCode::Success => return,
        ExitCode::UnexpectedToken => println!("hassl-Whops!: read an unexpected token."),
        ExitCode::EndOfFile => println!("hassl-Whops!: reached the end of the file."),
        ExitCode::UnselectedDirection => {
            println!("hassl-Whops!: attempted operation with unselected direction.")
        }
        ExitCode::StackUnderflow => println!("hassl-Whops!: attempted pop from empty stack."),
        ExitCode::Input => println!("hassl-Whops!: failed to get user input."),
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
