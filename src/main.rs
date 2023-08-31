use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{stdin, stdout, Read, Write},
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
    StateDoesNotExist,
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

fn terminate(_: &mut ProgramData, _: &Vec<char>) -> Option<ExitCode> {
    Option::Some(ExitCode::Success)
}

fn advance(program_data: &mut ProgramData, _: &Vec<char>) -> Option<ExitCode> {
    program_data.file_index += 1;
    Option::None
}

fn continue_if(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_register_value(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(value) => {
            return if value == 0 {
                go_to_state(program_data, file_contents)
            } else {
                advance(program_data, file_contents)
            }
        }
    }
}

fn continue_not(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_register_value(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(value) => {
            return if value == 0 {
                advance(program_data, file_contents)
            } else {
                go_to_state(program_data, file_contents)
            }
        }
    }
}

fn go_to_state(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.file_index = match file_contents
        .iter()
        .skip(program_data.file_index)
        .position(|c| *c == state_to_char(&program_data.selected_state))
    {
        None => {
            match file_contents
                .iter()
                .position(|c| *c == state_to_char(&program_data.selected_state))
            {
                None => return Option::Some(ExitCode::StateDoesNotExist),
                Some(value) => value + 1,
            }
        }
        Some(value) => program_data.file_index + value + 1,
    };
    Option::None
}

fn set_state(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
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
    advance(program_data, file_contents);
    Option::None
}

fn state_to_char(state: &State) -> char {
    return match state {
        State::State0 => '0',
        State::State1 => '1',
        State::State2 => '2',
        State::State3 => '3',
        State::State4 => '4',
        State::State5 => '5',
        State::State6 => '6',
        State::State7 => '7',
        State::State8 => '8',
        State::State9 => '9',
        State::StateA => 'A',
        State::StateB => 'B',
        State::StateC => 'C',
        State::StateD => 'D',
        State::StateE => 'E',
        State::StateF => 'F',
    };
}

fn push(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    let register_value = program_data.register_value;
    match get_stack(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => stack.push(register_value),
    }
    advance(program_data, file_contents);
    Option::None
}

fn pop(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack_value(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(value) => program_data.register_value = value,
    };
    advance(program_data, file_contents);
    Option::None
}

fn line_input(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
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
    advance(program_data, file_contents);
    Option::None
}

fn select_left(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.direction = Direction::Left;
    advance(program_data, file_contents);
    Option::None
}

fn select_right(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.direction = Direction::Right;
    advance(program_data, file_contents);
    Option::None
}

fn increment(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.register_value = match program_data.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => program_data.register_value.wrapping_add(16),
        Direction::Right => {
            (program_data.register_value / 16 * 16) + ((program_data.register_value % 16 + 1) % 16)
        }
    };
    advance(program_data, file_contents);
    Option::None
}

fn reset(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.register_value = match program_data.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => program_data.register_value % 16,
        Direction::Right => program_data.register_value / 16 * 16,
    };
    advance(program_data, file_contents);
    Option::None
}

fn add(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack_values(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            program_data.register_value = first_value.wrapping_add(second_value)
        }
    };
    advance(program_data, file_contents);
    Option::None
}

fn subtract(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack_values(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            program_data.register_value = first_value.wrapping_sub(second_value)
        }
    };
    advance(program_data, file_contents);
    Option::None
}

fn count(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => {
            program_data.register_value = match u8::try_from(stack.len()) {
                Err(_) => u8::MAX,
                Ok(value) => value,
            }
        }
    }
    advance(program_data, file_contents);
    Option::None
}

fn print(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    print!(
        "{}",
        if program_data.register_value.is_ascii() {
            program_data.register_value as char
        } else {
            char::REPLACEMENT_CHARACTER
        }
    );
    advance(program_data, file_contents);
    Option::None
}

fn numeric_print(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    print!("{}", program_data.register_value);
    advance(program_data, file_contents);
    Option::None
}

fn flush(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match stdout().flush() {
        Err(_) => return Option::Some(ExitCode::Internal),
        Ok(_) => (),
    }
    advance(program_data, file_contents);
    Option::None
}

fn generate_random(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.register_value = random();
    advance(program_data, file_contents);
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

fn get_register_value(program_data: &mut ProgramData) -> Result<u8, ExitCode> {
    match program_data.direction {
        Direction::Unselected => Err(ExitCode::UnselectedDirection),
        Direction::Left => Ok(program_data.register_value / 16),
        Direction::Right => Ok(program_data.register_value & 16),
    }
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
        ('f', flush),
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

    let exit_code = go_to_state(&mut program_data, &file_contents);

    let exit_code = if exit_code.is_some() {
        exit_code.unwrap()
    } else {
        loop {
            if file_contents.len() < program_data.file_index + 1 {
                break ExitCode::EndOfFile;
            }

            let function = match commands.get(&file_contents[program_data.file_index]) {
                None => break ExitCode::UnexpectedToken,
                Some(function) => function,
            };

            match function(&mut program_data, &file_contents) {
                None => (),
                Some(exit_code) => break exit_code,
            }
        }
    };

    match exit_code {
        ExitCode::Success => return,
        ExitCode::UnexpectedToken => println!("hassl-err!: read an unexpected token."),
        ExitCode::EndOfFile => println!("hassl-err!: reached the end of the file."),
        ExitCode::UnselectedDirection => {
            println!("hassl-err!: attempted operation with unselected direction.")
        }
        ExitCode::StackUnderflow => println!("hassl-err!: attempted pop from empty stack."),
        ExitCode::Input => println!("hassl-err!: failed to get user input."),
        ExitCode::Internal => println!("hassl-err!: an internal error occurred."),
        ExitCode::StateDoesNotExist => println!("hassl-err!: could not find the current state."),
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
