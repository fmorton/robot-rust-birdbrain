//mod device;

use crate::device::connected;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hummingbird_new() {
        let hummingbird = Hummingbird::new('A');

        assert_eq!(hummingbird.state.device, 'A');
        println!("{:?}", hummingbird.state.connected); //DEBUG
        assert!(hummingbird.state.connected);
    }
}
