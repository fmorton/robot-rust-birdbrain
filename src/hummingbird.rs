use crate::constant;
use crate::device::Device;
use crate::microbit::microbit;
use crate::request::request;
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

    pub fn sleep(&self, milliseconds: u64) {
        utility::sleep(milliseconds);
    }

    pub fn is_shaking(&self) -> bool {
        microbit::microbit_is_shaking(&self.state)
    }

    pub fn led(&self, port: i32, intensity: i32) -> bool {
        utility::validate_port(&port.to_string(), constant::VALID_LED_PORTS, false);

        let calculated_intensity =
            utility::bounds(utility::calculate_intensity(intensity), 0, 255).to_string();

        Device::response_status(&vec![
            "hummingbird",
            "out",
            "led",
            &port.to_string(),
            &calculated_intensity,
            &self.state.device.to_string(),
        ])
    }

    /// Set tri_led of a certain port requested to valid intensities.
    pub fn tri_led(&self, port: i32, r_intensity: i32, g_intensity: i32, b_intensity: i32) -> bool {
        utility::validate_port(&port.to_string(), constant::VALID_TRI_LED_PORTS, false);

        request::tri_led_response(
            self.state.device,
            port,
            r_intensity,
            g_intensity,
            b_intensity,
        )
    }

    ///Set Position servo of a certain port requested to a valid angle.
    pub fn position_servo(&self, port: i32, angle: i32) -> bool {
        utility::validate_port(&port.to_string(), constant::VALID_SERVO_PORTS, false);

        let calculated_angle = utility::bounds(utility::calculate_angle(angle), 0, 254).to_string();

        Device::response_status(&vec![
            "hummingbird",
            "out",
            "servo",
            &port.to_string(),
            &calculated_angle,
            &self.state.device.to_string(),
        ])
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
    fn test_hummingbird_sleep() {
        let hummingbird = Hummingbird::new('A');

        hummingbird.sleep(10);
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

    #[test]
    fn test_hummingbird_tri_led() {
        let hummingbird = Hummingbird::new('A');

        for _ in 0..3 {
            assert!(hummingbird.tri_led(1, 30, 30, 30));
            assert!(hummingbird.tri_led(1, 0, 0, 0));
        }
    }

    #[test]
    #[should_panic(expected = "Invalid Port: 7")]
    fn test_hummingbird_tri_led_invalid_port() {
        let hummingbird = Hummingbird::new('A');

        assert!(hummingbird.tri_led(7, 30, 30, 30));
    }

    #[test]
    fn test_hummingbird_position_servo() {
        let hummingbird = Hummingbird::new('A');

        assert!(hummingbird.position_servo(1, 50));

        hummingbird.sleep(150);

        assert!(hummingbird.position_servo(1, 130));
    }
}
