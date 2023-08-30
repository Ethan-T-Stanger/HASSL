use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

use rand::random;
use regex::Regex;

enum ExitCode {
    Success,
    UnexpectedToken,
    EndOfFile,
    UnselectedDirection,
    StackUnderflow,
    Input,
    Internal,
}

enum State {
    State0,
    State1,
    State2,
    State3,
    State4,
    State5,
    State6,
    State7,
    State8,
    State9,
    StateA,
    StateB,
    StateC,
    StateD,
    StateE,
    StateF,
}

enum Direction {
    Unselected,
    Left,
    Right,
}

struct ProgramData {
    file_index: usize,
    direction: Direction,
    left_stack: Vec<u8>,
    right_stack: Vec<u8>,
    register_value: u8,
    selected_state: State,
}

fn terminate(_: &mut ProgramData) -> Option<ExitCode> {
    Option::Some(ExitCode::Success)
}

fn advance(program_data: &mut ProgramData) -> Option<ExitCode> {
    program_data.file_index += 1;
    Option::None
}

fn set_state(program_data: &mut ProgramData) -> Option<ExitCode> {
    let value = match program_data.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => program_data.register_value / 16,
        Direction::Right => program_data.register_value % 16,
    };
    program_data.selected_state = match value {
        0 => State::State0,
        1 => State::State1,
        2 => State::State2,
        3 => State::State3,
        4 => State::State4,
        5 => State::State5,
        6 => State::State6,
        7 => State::State7,
        8 => State::State8,
        9 => State::State9,
        10 => State::StateA,
        11 => State::StateB,
        12 => State::StateC,
        13 => State::StateD,
        14 => State::StateE,
        15 => State::StateF,
        _ => return Option::Some(ExitCode::Internal),
    };
    advance(program_data);
    Option::None
}

fn push(program_data: &mut ProgramData) -> Option<ExitCode> {
    let register_value = program_data.register_value;
    match get_stack(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => stack.push(register_value),
    }
    advance(program_data);
    Option::None
}

fn pop(program_data: &mut ProgramData) -> Option<ExitCode> {
    match get_stack_value(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(value) => program_data.register_value = value,
    };
    advance(program_data);
    Option::None
}

fn line_input(program_data: &mut ProgramData) -> Option<ExitCode> {
    let stack = match get_stack(program_data) {
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
    advance(program_data);
    Option::None
}

fn select_left(program_data: &mut ProgramData) -> Option<ExitCode> {
    program_data.direction = Direction::Left;
    advance(program_data);
    Option::None
}

fn select_right(program_data: &mut ProgramData) -> Option<ExitCode> {
    program_data.direction = Direction::Right;
    advance(program_data);
    Option::None
}

fn increment(program_data: &mut ProgramData) -> Option<ExitCode> {
    program_data.register_value = match program_data.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => program_data.register_value.wrapping_add(16),
        Direction::Right => program_data.register_value.wrapping_add(1),
    };
    advance(program_data);
    Option::None
}

fn reset(program_data: &mut ProgramData) -> Option<ExitCode> {
    program_data.register_value = match program_data.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => program_data.register_value % 16,
        Direction::Right => program_data.register_value / 16 * 16,
    };
    advance(program_data);
    Option::None
}

fn add(program_data: &mut ProgramData) -> Option<ExitCode> {
    match get_stack_values(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            program_data.register_value = first_value.wrapping_add(second_value)
        }
    };
    advance(program_data);
    Option::None
}

fn subtract(program_data: &mut ProgramData) -> Option<ExitCode> {
    match get_stack_values(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            program_data.register_value = first_value.wrapping_sub(second_value)
        }
    };
    advance(program_data);
    Option::None
}

fn count(program_data: &mut ProgramData) -> Option<ExitCode> {
    match get_stack(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => {
            program_data.register_value = match u8::try_from(stack.len()) {
                Err(_) => u8::MAX,
                Ok(value) => value,
            }
        }
    }
    advance(program_data);
    Option::None
}

fn print(program_data: &mut ProgramData) -> Option<ExitCode> {
    print!(
        "{}",
        if program_data.register_value.is_ascii() {
            program_data.register_value as char
        } else {
            char::REPLACEMENT_CHARACTER
        }
    );
    advance(program_data);
    Option::None
}

fn numeric_print(program_data: &mut ProgramData) -> Option<ExitCode> {
    print!("{}", program_data.register_value);
    advance(program_data);
    Option::None
}

fn generate_random(program_data: &mut ProgramData) -> Option<ExitCode> {
    program_data.register_value = random();
    advance(program_data);
    Option::None
}

fn get_stack(program_data: &mut ProgramData) -> Result<&mut Vec<u8>, ExitCode> {
    let stack = match program_data.direction {
        Direction::Unselected => return Err(ExitCode::UnselectedDirection),
        Direction::Left => &mut program_data.left_stack,
        Direction::Right => &mut program_data.right_stack,
    };
    Ok(stack)
}

fn get_stack_value(program_data: &mut ProgramData) -> Result<u8, ExitCode> {
    let stack = match get_stack(program_data) {
        Err(exit_code) => return Err(exit_code),
        Ok(stack) => stack,
    };
    match stack.pop() {
        None => return Err(ExitCode::StackUnderflow),
        Some(value) => Ok(value),
    }
}

fn get_stack_values(program_data: &mut ProgramData) -> Result<(u8, u8), ExitCode> {
    let stack = match get_stack(program_data) {
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
        println!("hassl-Whops!: found an unclosed comment.");
        return;
    }

    let commands = HashMap::from([
        ('@', terminate as fn(&mut ProgramData) -> Option<ExitCode>),
        (' ', advance),
        ('$', set_state),
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
        direction: Direction::Unselected,
        left_stack: Vec::new(),
        right_stack: Vec::new(),
        register_value: 0,
        selected_state: State::State0,
    };

    let exit_code = loop {
        if file_contents.len() < program_data.file_index + 1 {
            break ExitCode::EndOfFile;
        }

        let function = match commands.get(&file_contents[program_data.file_index]) {
            None => break ExitCode::UnexpectedToken,
            Some(function) => function,
        };

        match function(&mut program_data) {
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
        ExitCode::Internal => println!("hassl-Whops!: an internal error occurred.")
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
