use crate::constant;

pub fn validate(validate: &str, validate_range: &str, validate_message: &str) -> bool {
    if !validate_range
        .to_lowercase()
        .contains(&validate.to_lowercase())
    {
        panic!("{}", validate_message.to_string());
    }

    true
}

pub fn validate_port(port: &str, valid_range: &str, allow_all: bool) -> bool {
    if allow_all && port.to_lowercase() == "all" {
        return true;
    }

    let validate_message = format!("Invalid Port: {}", port);

    return self::validate(port, valid_range, &validate_message);
}

pub fn bounds(utility_input: i32, input_min: i32, input_max: i32) -> i32 {
    if utility_input < input_min {
        return input_min;
    }
    if utility_input > input_max {
        return input_max;
    }

    utility_input
}

pub fn decimal_bounds(utility_input: f64, input_min: f64, input_max: f64) -> f64 {
    if utility_input < input_min {
        return input_min;
    }
    if utility_input > input_max {
        return input_max;
    }

    utility_input
}

pub fn calculate_angle(intensity: i32) -> i32 {
    intensity * 255 / 180
}

pub fn calculate_intensity(intensity: i32) -> i32 {
    bounds(intensity, 0, 100) * 255 / 100
}

pub fn calculate_speed(speed: i32) -> i32 {
    if speed >= -10 && speed <= 10 {
        return 255;
    }

    // QUESTION: why this calculation instead of normal mapping to 0..255 (and 255 means stop)
    // return ((int(speed) * 23 / 100) + 122)

    if speed < 0 {
        return 119 - (-speed / 100 * 45);
    }

    speed / 100 * 25 + 121
}

pub fn calculate_left_or_right(direction: &str) -> &str {
    if direction == constant::LEFT {
        return "Left";
    }

    if direction == constant::RIGHT {
        return "Right";
    }

    "None"
}

#[cfg(test)]
#[test]
fn test_validate() {
    assert!(self::validate(
        "A",
        constant::VALID_DEVICES,
        "Invalid Message"
    ));
    assert!(self::validate(
        "1",
        constant::VALID_TAIL_PORTS,
        "Invalid Message"
    ));
    assert!(self::validate(
        "all",
        constant::VALID_TAIL_PORTS,
        "Invalid Message"
    ));
    assert!(self::validate(
        "All",
        constant::VALID_TAIL_PORTS,
        "Invalid Message"
    ));

    assert!(self::validate_port("1", constant::VALID_TAIL_PORTS, true));
    assert!(self::validate_port("all", constant::VALID_TAIL_PORTS, true));
    assert!(self::validate_port("All", constant::VALID_TAIL_PORTS, true));

    let port = 1;
    assert!(self::validate_port(
        &1.to_string(),
        constant::VALID_TAIL_PORTS,
        true
    ));
}

#[test]
#[should_panic(expected = "Invalid Message")]
fn test_validate_panic() {
    assert!(!self::validate(
        "X",
        constant::VALID_DEVICES,
        "Invalid Message"
    ));
}

#[test]
#[should_panic(expected = "Invalid Port: 9")]
fn test_validate_port_allow_all_panic() {
    assert!(!self::validate_port("9", constant::VALID_SERVO_PORTS, true));
}

#[test]
#[should_panic(expected = "Invalid Port: J")]
fn test_validate_port_panic() {
    assert!(!self::validate_port(
        "J",
        constant::VALID_SERVO_PORTS,
        false
    ));
}

#[test]
fn test_bounds() {
    assert_eq!(bounds(10, 0, 100), 10);
    assert_eq!(bounds(10, 0, 100), 10);
    assert_eq!(bounds(10, 0, 100), 10);
    assert_eq!(bounds(10, -100, 100), 10);
    assert_eq!(bounds(-10, -100, 100), -10);
    assert_eq!(bounds(-100, -100, 100), -100);
    assert_eq!(bounds(100, -100, 100), 100);

    assert_eq!(bounds(101, -100, 100), 100);
    assert_eq!(bounds(-101, -100, 100), -100);
    assert_eq!(bounds(999999, -100, 100), 100);
    assert_eq!(bounds(-999999, -100, 100), -100);
}

#[test]
fn test_decimal_bounds() {
    assert_eq!(decimal_bounds(10.0, 0.0, 100.0), 10.0);
    assert_eq!(decimal_bounds(10.0, -100.0, 100.0), 10.0);
    assert_eq!(decimal_bounds(-10.0, -100.0, 100.0), -10.0);
    assert_eq!(decimal_bounds(-100.0, -100.0, 100.0), -100.0);
    assert_eq!(decimal_bounds(100.0, -100.0, 100.0), 100.0);

    assert_eq!(decimal_bounds(101.0, -100.0, 100.0), 100.0);
    assert_eq!(decimal_bounds(-101.0, -100.0, 100.0), -100.0);
    assert_eq!(decimal_bounds(999999.0, -100.0, 100.0), 100.0);
    assert_eq!(decimal_bounds(-999999.0, -100.0, 100.0), -100.0);
}

#[test]
fn test_calculate_angle() {}

#[test]
fn test_calculate_intensity() {}

#[test]
fn test_calculate_speed() {
    assert_eq!(calculate_speed(0), 255);
    assert_eq!(calculate_speed(9), 255);
    assert_eq!(calculate_speed(100), 146);
    assert_eq!(calculate_speed(-100), 74);
}

#[test]
fn test_calculate_left_or_right() {
    assert_eq!(calculate_left_or_right("L"), "Left");
    assert_eq!(calculate_left_or_right("R"), "Right");
    assert_eq!(calculate_left_or_right("BAD"), "None");
}
