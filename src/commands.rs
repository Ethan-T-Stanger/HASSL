use std::{
    io::{stdin, stdout, Write},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rand::random;

use crate::types::{Direction, ExitCode, ProgramData, State};

pub fn terminate(_: &mut ProgramData, _: &Vec<char>) -> Option<ExitCode> {
    Option::Some(ExitCode::Success)
}

pub fn advance(program_data: &mut ProgramData, _: &Vec<char>) -> Option<ExitCode> {
    program_data.file_index += 1;
    Option::None
}

pub fn continue_if(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
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

pub fn continue_not(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
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

pub fn go_to_state(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
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
                None => return Option::Some(ExitCode::StateUndefined),
                Some(value) => value + 1,
            }
        }
        Some(value) => program_data.file_index + value + 1,
    };
    Option::None
}

pub fn set_state(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    let value = match program_data.direction {
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
        _ => return Option::Some(ExitCode::InternalError),
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

pub fn push(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    let register_value = program_data.register_value;
    match get_stack(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => stack.push(register_value),
    }
    advance(program_data, file_contents);
    Option::None
}

pub fn pop(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack_value(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(value) => program_data.register_value = value,
    };
    advance(program_data, file_contents);
    Option::None
}

pub fn line_input(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    let stack = match get_stack(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok(stack) => stack,
    };
    let mut input_string = String::new();
    match stdin().read_line(&mut input_string) {
        Err(_) => return Option::Some(ExitCode::InternalError),
        Ok(_) => (),
    };
    input_string.chars().rev().for_each(|char| {
        stack.push(match u8::try_from(char) {
            Err(_) => char::REPLACEMENT_CHARACTER as u8,
            Ok(value) => value,
        })
    });
    advance(program_data, file_contents);
    Option::None
}

pub fn select_left(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.direction = Direction::Left;
    advance(program_data, file_contents);
    Option::None
}

pub fn select_right(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.direction = Direction::Right;
    advance(program_data, file_contents);
    Option::None
}

pub fn increment(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.register_value = match program_data.direction {
        Direction::Left => program_data.register_value.wrapping_add(16),
        Direction::Right => {
            (program_data.register_value / 16 * 16) + ((program_data.register_value % 16 + 1) % 16)
        }
    };
    advance(program_data, file_contents);
    Option::None
}

pub fn reset(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.register_value = match program_data.direction {
        Direction::Left => program_data.register_value % 16,
        Direction::Right => program_data.register_value / 16 * 16,
    };
    advance(program_data, file_contents);
    Option::None
}

pub fn add(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack_values(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            program_data.register_value = first_value.wrapping_add(second_value)
        }
    };
    advance(program_data, file_contents);
    Option::None
}

pub fn subtract(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    match get_stack_values(program_data) {
        Err(exit_code) => return Option::Some(exit_code),
        Ok((first_value, second_value)) => {
            program_data.register_value = first_value.wrapping_sub(second_value)
        }
    };
    advance(program_data, file_contents);
    Option::None
}

pub fn count(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
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

pub fn print(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    print!(
        "{}",
        if program_data.register_value.is_ascii() {
            program_data.register_value as char
        } else {
            char::REPLACEMENT_CHARACTER
        }
    );
    advance(program_data, file_contents);
    flush()
}

pub fn numeric_print(
    program_data: &mut ProgramData,
    file_contents: &Vec<char>,
) -> Option<ExitCode> {
    print!("{:X}", program_data.register_value);
    advance(program_data, file_contents);
    flush()
}

pub fn time(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    program_data.register_value = match program_data.start_time.duration_since(UNIX_EPOCH) {
        Err(_) => return Option::Some(ExitCode::InternalError),
        Ok(value) => (value.as_millis() % 256) as u8,
    };
    program_data.start_time = SystemTime::now();
    advance(program_data, file_contents);
    Option::None
}

pub fn wait(program_data: &mut ProgramData, file_contents: &Vec<char>) -> Option<ExitCode> {
    thread::sleep(Duration::from_millis(program_data.register_value as u64));
    advance(program_data, file_contents);
    Option::None
}

fn flush() -> Option<ExitCode> {
    match stdout().flush() {
        Err(_) => return Option::Some(ExitCode::InternalError),
        Ok(_) => (),
    }
    Option::None
}

pub fn generate_random(
    program_data: &mut ProgramData,
    file_contents: &Vec<char>,
) -> Option<ExitCode> {
    program_data.register_value = random();
    advance(program_data, file_contents);
    Option::None
}

fn get_stack(program_data: &mut ProgramData) -> Result<&mut Vec<u8>, ExitCode> {
    let stack = match program_data.direction {
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
        Direction::Left => Ok(program_data.register_value / 16),
        Direction::Right => Ok(program_data.register_value % 16),
    }
}
