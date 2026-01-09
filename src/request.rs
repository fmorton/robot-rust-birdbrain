pub mod request {
    use crate::device::Device;
    use crate::utility;

    /// Set tri_led of a certain port requested to valid intensities.
    pub fn tri_led_response(
        device: char,
        port: i32,
        r_intensity: i32,
        g_intensity: i32,
        b_intensity: i32,
    ) -> bool {
        let calculated_intensity_red = utility::calculate_intensity(r_intensity);
        let calculated_intensity_green = utility::calculate_intensity(g_intensity);
        let calculated_intensity_blue = utility::calculate_intensity(b_intensity);

        Device::response_status(&vec![
            "hummingbird",
            "out",
            "triled",
            &port.to_string(),
            &calculated_intensity_red.to_string(),
            &calculated_intensity_green.to_string(),
            &calculated_intensity_blue.to_string(),
            &device.to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tri_led_response() {
        request::tri_led_response('A', 1, 50, 0, 0);
    }

    #[test]
    #[should_panic(expected = "Device Not Connected: C")]
    fn test_response_using_uri_no_device() {
        request::tri_led_response('C', 1, 50, 0, 0);
    }

    #[test]
    #[should_panic(expected = "Device Not Connected: A")]
    fn test_response_using_uri_invalid_port() {
        request::tri_led_response('A', 8, 50, 0, 0);
    }
}
