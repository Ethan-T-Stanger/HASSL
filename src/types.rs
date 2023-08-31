pub enum ExitCode {
    Success,
    EndOfFile,
    TokenUndefined,
    StateUndefined,
    StackUnderflow,
    InternalError,
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
}
