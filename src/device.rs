//mod state;

pub struct Device {
    pub state: u32,
    pub device: char,
    pub connected: bool,
}

pub trait Device {
    fn new(device: char) -> Device {
        Device { device: device, state: 0, connected: false }
    }
    fn is_connected(&self) -> bool {
        return self.connected
    }
}
