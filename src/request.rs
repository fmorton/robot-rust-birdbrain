pub struct Request {
    pub body: String,
    pub status: u16,
}

impl Request {
    fn request_uri_from_vector(request: &[&str]) -> String {
        return "http://127.0.0.1:30061/".to_string() + &request.join("/")
    }

    fn response_from_uri(uri: &String) -> Request {
        println!("{}", &uri); //DEBUG
        let response = reqwest::blocking::get(uri);

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let status = response.status().as_u16();
                    match response.text() {
                        Ok(text) => return Request { body: text, status: status },
                        Err(e) => eprintln!("Error reading body: {}", e),
                    }
                } else {
                    eprintln!("API returned non-success status: {}", response.status());
                }
            },
            Err(err) => {
                eprintln!("Request failed: {}", err);
            }
        }

        Request { body: "".to_string(), status: 500 }
    }

    pub fn response(request: &[&str]) -> Request {
        let uri = Request::request_uri_from_vector(request).to_string();

        return Request::response_from_uri(&uri)
    }

    pub fn response_status(request: &[&str]) -> bool {
        return Request::request_status(&Request::response(request).body);
    }

    pub fn is_not_connected_response(response: &str) -> bool {
        return response.to_lowercase() == "not connected"
    }

    pub fn request_status(status: &str) -> bool {
        if crate::constant::BIRDBRAIN_TEST {
            println!("Test: request status is {}", status)
        }

        //if status is None:
            //return None

        match status {
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

            _ => { panic!("unknown status"); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_using_uri() {
        let uri = Request::request_uri_from_vector(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

        assert_eq!(uri, "http://127.0.0.1:30061/hummingbird/in/orientation/Shake/A");

        let response_using_uri = Request::response_from_uri(&uri);

        assert_eq!(response_using_uri.status, 200);
        assert_eq!(response_using_uri.body, "false");
    }

    #[test]
    fn test_response(){
        let response = Request::response(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

        assert_eq!(response.status, 200);
        assert_eq!(response.body, "false");

        assert!(!Request::is_not_connected_response(&response.body));
    }

    #[test]
    fn test_is_not_connected_response(){
        assert!(Request::is_not_connected_response("Not Connected"));
        assert!(Request::is_not_connected_response("Not connected"));
        assert!(Request::is_not_connected_response("not connected"));
        assert!(!Request::is_not_connected_response("Something Else"));

        let response = Request::response(&vec!["hummingbird", "in", "orientation", "Shake", "C"]);

        assert!(Request::is_not_connected_response(&response.body));
    }

    #[test]
    fn test_request_status() {
        assert!(Request::request_status("true"));
        assert!(Request::request_status("true"));
        assert!(Request::request_status("true"));
        assert!(Request::request_status("led set"));
        assert!(Request::request_status("triled set"));
        assert!(Request::request_status("servo set"));
        assert!(Request::request_status("buzzer set"));
        assert!(Request::request_status("symbol set"));
        assert!(Request::request_status("print set"));
        assert!(Request::request_status("all stopped"));

        assert!(Request::request_status("finch moved"));
        assert!(Request::request_status("finch turned"));
        assert!(Request::request_status("finch wheels started"));
        assert!(Request::request_status("finch wheels stopped"));
        assert!(Request::request_status("finch encoders reset"));

        assert!(!Request::request_status("false"));
        assert!(!Request::request_status("not connected"));
        assert!(!Request::request_status("invalid orientation"));
        assert!(!Request::request_status("invalid port"));
    }
}
