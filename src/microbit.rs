pub mod microbit {
    use crate::device::Device;
    use crate::state::State;

    pub fn microbit_is_shaking(state: &State) -> bool {
        let response = Device::response(&vec![
            "hummingbird",
            "in",
            "orientation",
            "Shake",
            &state.device.to_string(),
        ]);

        return response.body == "true";
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::microbit::microbit::microbit_is_shaking;
    use crate::state::State;

    #[test]
    fn test_microbit_is_shaking() {
        let state = State::new('A');

        assert!(!microbit_is_shaking(&state));
    }
}
