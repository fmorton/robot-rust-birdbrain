pub struct Request {
    pub body: String,
    pub status: u16,
}

impl Request {
    fn request_uri_from_vector(request: &[&str]) -> String {
        return "http://127.0.0.1:30061/".to_string() + &request.join("/")
    }

    fn response_from_uri(uri: &String) -> Request {
        println!("{}", &uri);
        let response = reqwest::blocking::get(uri);

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let status = response.status().as_u16();
//println!("{}", response.text().to_string());  //DEBUG
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
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::Request;

    #[test]
    fn always_true() {
        assert!(true)
    }
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
    }
}
