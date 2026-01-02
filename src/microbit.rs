pub mod microbit {
    use crate::state::State;
    use crate::request::Request;
    pub fn microbit_is_shaking(state: &State) -> bool {
        println!("DEBUG::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        println!("{}", state.device);
        println!("{}", state.connected);
        let response = Request::response(&vec!["hummingbird", "in", "orientation", "Shake", &state.device.to_string()]);
println!("{}", "show the actual body of the response.................");
        println!("{}", response.body);

        return response.body == "true"
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
