pub struct State {
    pub device: char,
    pub connected: bool,
    pub cache: u32,
    pub display_map: [bool; 25],
}

impl State {
    pub fn new(device: char) -> State {
        //pub fn new(device: char) -> State {
        State {
            device: device,
            connected: false,
            cache: 0,
            display_map: State::microbit_empty_display_map()
        }
    }

    fn microbit_empty_display_map() -> [bool; 25] {
      [false; 25]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::State;

    #[test]
    fn test_initial_display_map() {
        let state = State::new('A');
        for i in 0..25 {
            assert_eq!(state.display_map[i], false);
        }
    }
}
