use crate::state::State;

pub struct Finch {
    state: State,
}

impl Finch {
    fn new(device: char) -> Finch {
        let finch = Finch { state: State::new(device) };

        finch
    }
}
