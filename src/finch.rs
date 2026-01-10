use crate::state::State;
use crate::utility;

pub struct Finch {
    state: State,
}

impl Finch {
    fn new(device: char) -> Finch {
        let finch = Finch {
            state: State::new(device),
        };

        finch
    }

    pub fn sleep(&self, milliseconds: u64) {
        utility::sleep(milliseconds);
    }
}
