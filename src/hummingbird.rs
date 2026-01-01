use crate::device::connected;
//use crate::microbit::microbit_is_shaking;
//use crate::microbit::Microbit::is_shaking;
//use crate::microbit::Microbit::*;
use crate::state::State;

pub struct Hummingbird {
    state: State,
}

impl Hummingbird {
    fn new(device: char) -> Hummingbird {
        let mut hummingbird = Hummingbird { state: State::new(device)};

        hummingbird.state.connected = connected(&mut hummingbird.state);

        hummingbird
    }

    pub fn is_shaking(&self) -> bool {
        //microbit_is_shaking(&mut self.state)
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hummingbird_new() {
        let hummingbird = Hummingbird::new('A');
        assert_eq!(hummingbird.state.device, 'A');
        assert!(hummingbird.state.connected);
    }

    #[test]
    fn test_hummingbird_is_shaking() {
        let hummingbird = Hummingbird::new('A');

        assert!(!hummingbird.is_shaking());
    }

}
