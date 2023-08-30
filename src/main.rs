use std::{collections::HashMap, env::args, fs::File, io::Read, path::PathBuf};

enum ExitCode {
    Success,
    UnexpectedToken,
    EndOfFile,
    UnselectedDirection,
    StackUnderflow,
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
    match state.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => state.left_stack.push(state.register_value),
        Direction::Right => state.right_stack.push(state.register_value),
    }
    advance(state);
    Option::None
}

fn pop(state: &mut State) -> Option<ExitCode> {
    match state.direction {
        Direction::Unselected => return Option::Some(ExitCode::UnselectedDirection),
        Direction::Left => {
            state.register_value = match state.left_stack.pop() {
                Option::None => return Option::Some(ExitCode::StackUnderflow),
                Option::Some(value) => value,
            }
        }
        Direction::Right => {
            state.register_value = match state.right_stack.pop() {
                Option::None => return Option::Some(ExitCode::StackUnderflow),
                Option::Some(value) => value,
            }
        }
    }
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
        ('<', select_left),
        ('>', select_right),
        ('*', increment),
        ('.', reset),
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
