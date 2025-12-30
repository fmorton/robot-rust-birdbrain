use std::error::Error;

struct Request {
    body: String,
    status: u16,
}

impl Request {
    fn request_uri_from_vector(request: &[&str]) -> String {
        return "http://127.0.0.1:30061/".to_string() + &request.join("/")
    }

    fn response_from_uri(uri: &String) -> Request {
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

    fn response(request: &[&str]) -> Request {
        let uri = Request::request_uri_from_vector(request).to_string();

        return Request::response_from_uri(&uri)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let uri = Request::request_uri_from_vector(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

    let response_using_uri = Request::response_from_uri(&uri);

    println!("URI: {}", uri);
    println!("Status: {}", response_using_uri.status);
    println!("Body: {}", response_using_uri.body);

    let response = Request::response(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

    println!("Status: {}", response.status);
    println!("Body: {}", response.body);

    Ok(())
}
