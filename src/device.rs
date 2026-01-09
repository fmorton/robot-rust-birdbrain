use crate::constant;

pub struct Device {
    pub body: String,
    pub status: u16,
}

impl Device {
    pub fn connected(device: char) -> bool {
        let response = Device::response(&vec![
            "hummingbird",
            "in",
            "orientation",
            "Shake",
            &device.to_string(),
        ]);

        return response.body == "false";
    }

    fn error_response() -> Device {
        Device {
            body: "".to_string(),
            status: 500,
        }
    }

    fn device_not_connected_panic(device: char) {
        panic!("Device Not Connected: {}", device);
    }

    fn request_uri_from_vector(request: &[&str]) -> String {
        let response = "http://127.0.0.1:30061/".to_string() + &request.join("/");

        response
    }

    fn response_from_uri(uri: &String) -> Device {
        let response = reqwest::blocking::get(uri);

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let status = response.status().as_u16();
                    match response.text() {
                        Ok(text) => {
                            if Device::is_not_connected_response(&text) {
                                Device::device_not_connected_panic(
                                    Device::extracted_device_from_uri(uri),
                                );
                            }

                            return Device {
                                body: text,
                                status: status,
                            };
                        }
                        Err(e) => return Device::error_response(),
                    }
                } else {
                    return Device::error_response();
                }
            }
            Err(err) => {
                panic!("Bluebird Connector Not Running");
            }
        }
    }

    pub fn response(request: &[&str]) -> Device {
        let uri = Device::request_uri_from_vector(request).to_string();

        let response = Device::response_from_uri(&uri);

        if Device::is_not_connected_response(&response.body) {
            Device::device_not_connected_panic(Device::extracted_device(request));
        }

        response
    }

    pub fn response_status(request: &[&str]) -> bool {
        return Device::request_status(&Device::response(request).body);
    }

    pub fn is_not_connected_response(response: &str) -> bool {
        return response.to_lowercase() == "not connected";
    }

    pub fn request_status(status: &str) -> bool {
        if crate::constant::BIRDBRAIN_TEST {
            println!("Test: request status is {}", status)
        }

        match status.to_lowercase().as_str() {
            "true" => return true,
            "led set" => return true,
            "triled set" => return true,
            "servo set" => return true,
            "buzzer set" => return true,
            "symbol set" => return true,
            "print set" => return true,
            "all stopped" => return true,

            "finch moved" => return true,
            "finch turned" => return true,
            "finch wheels started" => return true,
            "finch wheels stopped" => return true,
            "finch encoders reset" => return true,

            "false" => return false,
            "not connected" => return false,
            "invalid orientation" => return false,
            "invalid port" => return false,

            _ => {
                panic!("Unknown Status: {}", status);
            }
        }
    }

    pub fn extracted_device_from_uri(uri: &str) -> char {
        for device in uri.split('/').rev() {
            if device.len() == 0 {
                continue;
            }
            if constant::VALID_DEVICES.contains(device) {
                return device.chars().nth(0).unwrap();
            }
        }
        constant::UNKNOWN_DEVICE
    }
    pub fn extracted_device(request: &[&str]) -> char {
        Device::extracted_device_from_uri(&Device::request_uri_from_vector(request))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_using_uri() {
        let uri = Device::request_uri_from_vector(&vec![
            "hummingbird",
            "in",
            "orientation",
            "Shake",
            "A",
        ]);

        assert_eq!(
            uri,
            "http://127.0.0.1:30061/hummingbird/in/orientation/Shake/A"
        );
        let response_using_uri = Device::response_from_uri(&uri);

        assert_eq!(response_using_uri.status, 200);
        assert_eq!(response_using_uri.body, "false");
    }

    #[test]
    #[should_panic(expected = "Device Not Connected: C")]
    fn test_response_using_uri_no_device() {
        let uri = Device::request_uri_from_vector(&vec![
            "hummingbird",
            "in",
            "orientation",
            "Shake",
            "C",
        ]);

        Device::response_from_uri(&uri);
    }
    #[test]
    #[should_panic(expected = "Device Not Connected: ?")]
    fn test_response_using_uri_invalid_device() {
        let uri = Device::request_uri_from_vector(&vec![
            "hummingbird",
            "in",
            "orientation",
            "Shake",
            "Z",
        ]);

        Device::response_from_uri(&uri);
    }
    #[test]
    fn test_response() {
        let response = Device::response(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

        assert_eq!(response.status, 200);
        assert_eq!(response.body, "false");

        assert!(!Device::is_not_connected_response(&response.body));
    }

    #[test]
    #[should_panic(expected = "Device Not Connected: C")]
    fn test_response_no_device() {
        let response = Device::response(&vec!["hummingbird", "in", "orientation", "Shake", "C"]);
    }

    #[test]
    fn test_is_not_connected_response() {
        assert!(Device::is_not_connected_response("Not Connected"));
        assert!(Device::is_not_connected_response("Not connected"));
        assert!(Device::is_not_connected_response("not connected"));
        assert!(!Device::is_not_connected_response("Something Else"));
    }

    #[test]
    fn test_request_status() {
        assert!(Device::request_status("true"));
        assert!(Device::request_status("true"));
        assert!(Device::request_status("true"));
        assert!(Device::request_status("led set"));
        assert!(Device::request_status("triled set"));
        assert!(Device::request_status("servo set"));
        assert!(Device::request_status("buzzer set"));
        assert!(Device::request_status("symbol set"));
        assert!(Device::request_status("print set"));
        assert!(Device::request_status("all stopped"));

        assert!(Device::request_status("finch moved"));
        assert!(Device::request_status("finch turned"));
        assert!(Device::request_status("finch wheels started"));
        assert!(Device::request_status("finch wheels stopped"));
        assert!(Device::request_status("finch encoders reset"));

        assert!(!Device::request_status("false"));
        assert!(!Device::request_status("not connected"));
        assert!(!Device::request_status("invalid orientation"));
        assert!(!Device::request_status("invalid port"));
    }

    #[test]
    #[should_panic(expected = "Unknown Status: ")]
    fn test_request_status_should_panic_empty() {
        assert!(!Device::request_status(""));
    }

    #[test]
    #[should_panic(expected = "Unknown Status: nonesense")]
    fn test_request_status_should_panic_nonsense() {
        assert!(!Device::request_status("nonesense"));
    }

    #[test]
    fn test_extracted_device() {
        assert!(
            'A' == Device::extracted_device(&vec![
                "hummingbird",
                "in",
                "orientation",
                "Shake",
                "A"
            ])
        );
        assert!('A' == Device::extracted_device(&vec!["B", "in", "orientation", "Shake", "A"]));
        assert!('C' == Device::extracted_device(&vec!["hummingbird", "C", "orientation", "Shake"]));
        assert!(
            constant::UNKNOWN_DEVICE
                == Device::extracted_device(&vec!["hummingbird", "in", "orientation", "Shake"])
        );

        assert!(
            Device::extracted_device(&vec!["hummingbird", "in", "orientation", "Shake", "A"])
                == 'A'
        );
        //assert!(Device::extracted_device(&vec![["hummingbird", "in", "orientation", "Shake", "A"]]) == 'A');
        //assert!(Device::extracted_device(&vec![("hummingbird", "in", "orientation", "Shake", "A")]) == 'A');
        //assert!(Device::extracted_device(&vec![[("hummingbird", "in", "orientation", "Shake", "A")]) == 'A');
        //assert!(Device::extracted_device((&vec![["hummingbird", "in", "orientation", "Shake", "A"])) == 'A');

        assert!(
            Device::extracted_device(&vec![
                "hummingbird",
                "out",
                "symbol",
                "C",
                "true/false/true/false"
            ]) == 'C'
        );
        assert!(
            Device::extracted_device(&vec![
                "hummingbird",
                "out",
                "symbol",
                "C",
                "false/true/false/true"
            ]) == 'C'
        );

        assert!(
            Device::extracted_device(&vec![
                "hummingbird",
                "out",
                "move",
                "B",
                "Forward",
                "7",
                "5"
            ]) == 'B'
        );
    }

    #[test]
    fn test_extracted_device_from_uri() {
        let uri = "http://127.0.0.1:30061/hummingbird/in/orientation/Shake/B";

        assert!(Device::extracted_device_from_uri(&uri) == 'B');

        let uri = "http://127.0.0.1:30061/hummingbird/out/symbol/C/false/true/false/true";

        assert!(Device::extracted_device_from_uri(&uri) == 'C');
    }
}
