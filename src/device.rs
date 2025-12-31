use crate::request::Request;
use crate::state::State;

pub fn connected(state: &mut State) -> bool {
    let response = Request::response(&vec!["hummingbird", "in", "orientation", "Shake", &state.device.to_string()]);

    return response.body == "false"
}
