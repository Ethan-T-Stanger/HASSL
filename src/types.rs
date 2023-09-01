use std::time::SystemTime;

pub enum ExitCode {
    Success,
    EndOfFile,
    TokenUndefined,
    StateUndefined,
    StackUnderflow,
    InternalError,
}

impl ExitCode {
    pub fn handle_error(&self) {
        let prefix = "hassl-err!:";
        match self {
            ExitCode::Success => return,
            ExitCode::EndOfFile => eprintln!("{} end of file", prefix),
            ExitCode::TokenUndefined => eprintln!("{} token undefined", prefix),
            ExitCode::StateUndefined => eprintln!("{} state undefined", prefix),
            ExitCode::StackUnderflow => eprintln!("{} stack underflow", prefix),
            ExitCode::InternalError => eprintln!("{} internal error occurred", prefix),
        }
    }
}

pub enum State {
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

pub enum Direction {
    Left,
    Right,
}

pub struct ProgramData {
    pub file_index: usize,
    pub direction: Direction,
    pub left_stack: Vec<u8>,
    pub right_stack: Vec<u8>,
    pub register_value: u8,
    pub selected_state: State,
    pub start_time: SystemTime,
}
