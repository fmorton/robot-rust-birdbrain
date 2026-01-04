pub fn validate(validate: &str, validate_range: &str, validate_message: &str) -> bool {
    if !validate_range.to_lowercase().contains(&validate.to_lowercase()) {
        panic!("{}", validate_message.to_string());
    }

    true
}

pub fn validate_port(port: &str, valid_range: &str, allow_all: bool) -> bool {
    if allow_all && port.to_lowercase() == "all" {
        return true;
    }

    let validate_message= format!("Invalid Port: {}", port);

    return self::validate(port, valid_range, &validate_message);
}

#[cfg(test)]
use crate::constant;

#[test]
fn test_validate() {
    assert!(self::validate("A", constant::VALID_DEVICES, "Invalid Message"));
    assert!(self::validate("1", constant::VALID_TAIL_PORTS, "Invalid Message"));
    assert!(self::validate("all", constant::VALID_TAIL_PORTS, "Invalid Message"));
    assert!(self::validate("All", constant::VALID_TAIL_PORTS, "Invalid Message"));

    assert!(self::validate_port("1", constant::VALID_TAIL_PORTS, true));
    assert!(self::validate_port("all", constant::VALID_TAIL_PORTS, true));
    assert!(self::validate_port("All", constant::VALID_TAIL_PORTS, true));

    let port = 1;
    assert!(self::validate_port(&1.to_string(), constant::VALID_TAIL_PORTS, true));

}

#[test]
#[should_panic(expected = "Invalid Message")]
fn test_validate_panic() {
    assert!(!self::validate("X", constant::VALID_DEVICES, "Invalid Message"));
}

#[test]
#[should_panic(expected = "Invalid Port: 9")]
fn test_validate_port_allow_all_panic() {
    assert!(!self::validate_port("9", constant::VALID_SERVO_PORTS, true));
}

#[test]
#[should_panic(expected = "Invalid Port: J")]
fn test_validate_port_panic() {
    assert!(!self::validate_port("J", constant::VALID_SERVO_PORTS, false));
}