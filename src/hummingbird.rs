use crate::constant;
use crate::device::Device;
use crate::microbit::microbit::microbit_is_shaking;
use crate::state::State;
use crate::utility;

pub struct Hummingbird {
    state: State,
}

impl Hummingbird {
    fn new(device: char) -> Hummingbird {
        let mut hummingbird = Hummingbird {
            state: State::new(device),
        };

        hummingbird.state.connected = Device::connected(device);

        hummingbird
    }

    pub fn is_shaking(&self) -> bool {
        microbit_is_shaking(&self.state)
    }

    pub fn led(&self, port: i32, intensity: i32) -> bool {
        utility::validate_port(&port.to_string(), constant::VALID_LED_PORTS, false);

        let calculated_intensity =
            utility::bounds(utility::calculate_intensity(intensity), 0, 255).to_string();

        return Device::response_status(&vec![
            "hummingbird",
            "out",
            "led",
            &port.to_string(),
            &calculated_intensity,
            &self.state.device.to_string(),
        ]);
    }

    /// Set tri_led of a certain port requested to valid intensities.
    pub fn tri_led(&self, port: i32, r_intensity: f64, g_intensity: f64, b_intensity: f64) -> bool {
        utility::validate_port(&port.to_string(), constant::VALID_TRI_LED_PORTS, false);

        //Device::tri_led_response(&self.state.device.to_string(), port, r_intensity, g_intensity, b_intensity)
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

    #[test]
    fn test_hummingbird_led() {
        let hummingbird = Hummingbird::new('A');

        for _ in 0..3 {
            assert!(hummingbird.led(1, 80));
            assert!(hummingbird.led(1, 0));
        }
    }

    #[test]
    #[should_panic(expected = "Invalid Port: 6")]
    fn test_hummingbird_led_invalid_port() {
        let hummingbird = Hummingbird::new('A');

        assert!(hummingbird.led(6, 80));
    }
}
